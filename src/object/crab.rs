use bevy::prelude::*;
use spew::prelude::*;

use crate::{
    common::{
        collider::{Collider, ColliderSet},
        fade::{Fade, FadeAnimation, FadeBundle},
        movement::{
            Acceleration, AccelerationBundle, Force, Heading, MaxSpeed,
            Movement, MovementSet, Speed, StoppingDistance, VelocityBundle,
        },
    },
    game::{
        assets::{CachedAssets, GameAssets, GameConfig, Player},
        competitors::GameMode,
        state::GameState,
    },
    level::{
        barrier::BARRIER_RADIUS,
        goal::{Goal, GOAL_WIDTH},
        side::Side,
    },
    object::ball::BALL_RADIUS,
    player::{ai::PlayerAi, input::PlayerInput},
    util::hemisphere_deflection,
};

use super::{ball::Ball, Object};

pub const CRAB_WIDTH: f32 = 0.2;
pub const CRAB_DEPTH: f32 = 0.1;
pub const CRAB_START_POSITION: Vec3 = Vec3::new(0.0, 0.05, 0.0);
pub const CRAB_POSITION_X_MAX: f32 =
    (0.5 * GOAL_WIDTH) - BARRIER_RADIUS - (0.5 * CRAB_WIDTH);

/// Makes a crab entity that can deflect balls and move sideways inside a goal.
#[derive(Component, Debug)]
pub struct Crab;

pub(super) struct CrabPlugin;

impl Plugin for CrabPlugin {
    fn build(&self, app: &mut App) {
        app.add_spawner((Object::Crab, spawn_crab_on_side))
            .add_systems(
                Update,
                (
                    restrict_crab_movement_range.after(MovementSet),
                    (
                        add_crab_movement_after_its_finished_fading_in,
                        remove_crab_movement_and_collision_when_fading_out,
                    )
                        .run_if(not(in_state(GameState::Paused))),
                ),
            )
            .add_systems(
                PostUpdate,
                crab_and_ball_collisions.in_set(ColliderSet),
            );
    }
}

fn spawn_crab_on_side(
    In(side): In<Side>,
    cached_assets: Res<CachedAssets>,
    game_mode: Res<GameMode>,
    game_assets: Res<GameAssets>,
    game_configs: Res<Assets<GameConfig>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    goals_query: Query<(Entity, &Side), With<Goal>>,
) {
    let game_config = game_configs.get(&game_assets.game_config).unwrap();
    let crab_config = &game_config.modes[game_mode.0].competitors[&side];
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(game_assets.image_crab.clone()),
        base_color: Color::hex(&crab_config.color).unwrap(),
        ..default()
    });
    let (goal_entity, _) = goals_query
        .iter()
        .find(|(_, goal_side)| **goal_side == side)
        .unwrap();

    commands.entity(goal_entity).with_children(|builder| {
        let mut crab = builder.spawn((
            Crab,
            Collider,
            side,
            FadeBundle {
                fade_animation: FadeAnimation::Scale {
                    max_scale: Vec3::new(CRAB_WIDTH, CRAB_DEPTH, CRAB_DEPTH),
                    axis_mask: Vec3::ONE,
                },
                ..default()
            },
            AccelerationBundle {
                velocity: VelocityBundle {
                    heading: Heading(Vec3::X),
                    ..default()
                },
                max_speed: MaxSpeed(crab_config.max_speed),
                acceleration: Acceleration(
                    crab_config.max_speed / crab_config.seconds_to_max_speed,
                ),
                ..default()
            },
            PbrBundle {
                mesh: cached_assets.crab_mesh.clone(),
                material: material_handle,
                transform: Transform::from_matrix(
                    Mat4::from_scale_rotation_translation(
                        Vec3::splat(f32::EPSILON),
                        Quat::IDENTITY,
                        CRAB_START_POSITION,
                    ),
                ),

                ..default()
            },
        ));

        if crab_config.player == Player::AI {
            crab.insert(PlayerAi);
        } else {
            crab.insert(PlayerInput);
        }
    });

    info!("Crab({:?}): Spawned", side);
}

fn add_crab_movement_after_its_finished_fading_in(
    mut commands: Commands,
    mut removed: RemovedComponents<Fade>,
    query: Query<Entity, With<Crab>>,
) {
    for entity in removed.iter() {
        if query.contains(entity) {
            commands.entity(entity).insert(Movement);
        }
    }
}

fn remove_crab_movement_and_collision_when_fading_out(
    mut commands: Commands,
    query: Query<(Entity, &Fade), (With<Crab>, Added<Fade>)>,
) {
    for (entity, fade) in &query {
        if matches!(fade, Fade::Out(_)) {
            commands
                .entity(entity)
                .remove::<Movement>()
                .remove::<Collider>();
        }
    }
}

fn restrict_crab_movement_range(
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut Transform, &mut Speed, &mut StoppingDistance),
        (With<Crab>, With<Movement>),
    >,
) {
    for (entity, mut transform, mut speed, mut stopping_distance) in &mut query
    {
        // Limit crab to bounds of the goal.
        if !(-CRAB_POSITION_X_MAX..=CRAB_POSITION_X_MAX)
            .contains(&transform.translation.x)
        {
            transform.translation.x = transform
                .translation
                .x
                .clamp(-CRAB_POSITION_X_MAX, CRAB_POSITION_X_MAX);
            speed.0 = 0.0;
            commands.entity(entity).remove::<Force>();
        }

        // Limit stopping distance to the bounds of the goal.
        let stopped_position = transform.translation.x + stopping_distance.0;

        if !(-CRAB_POSITION_X_MAX..=CRAB_POSITION_X_MAX)
            .contains(&stopped_position)
        {
            stopping_distance.0 = stopped_position.signum()
                * CRAB_POSITION_X_MAX
                - transform.translation.x;
        }
    }
}

fn crab_and_ball_collisions(
    mut commands: Commands,
    balls_query: Query<
        (Entity, &GlobalTransform, &Heading),
        (With<Ball>, With<Collider>, With<Movement>),
    >,
    crabs_query: Query<(&Side, &Transform), (With<Crab>, With<Collider>)>,
) {
    for (ball_entity, ball_transform, ball_heading) in &balls_query {
        for (side, crab_transform) in &crabs_query {
            // Check that the ball is touching the crab and facing the goal.
            let axis = side.axis();
            let ball_to_goal_distance = side.distance_to_ball(ball_transform);
            let ball_goal_position = side.get_ball_position(ball_transform);
            let delta = crab_transform.translation.x - ball_goal_position;
            let ball_to_crab_distance = delta.abs();

            if ball_to_goal_distance > BALL_RADIUS + (0.5 * CRAB_DEPTH)
                || ball_to_crab_distance > BALL_RADIUS + (0.5 * CRAB_WIDTH)
                || ball_heading.0.dot(axis) <= 0.0
            {
                continue;
            }

            let ball_deflection_direction =
                hemisphere_deflection(delta, CRAB_WIDTH, axis);

            commands
                .entity(ball_entity)
                .insert(Heading(ball_deflection_direction));
            info!("Ball({:?}): Collided Crab({:?})", ball_entity, side);
            break;
        }
    }
}
