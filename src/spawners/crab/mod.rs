pub mod ai;
pub mod player;

use bevy::prelude::*;

use crate::{
    common::{
        collider::{CircleCollider, Collider},
        fade::{Fade, FadeAnimation, InsertAfterFadeIn, RemoveBeforeFadeOut},
        movement::{
            Acceleration, Force, Heading, MaxSpeed, Movement, Speed,
            StoppingDistance,
        },
    },
    game::{
        assets::{CachedAssets, CrabController, GameAssets},
        modes::GameModes,
        state::{GameState, PausableSet},
    },
    level::{
        beach::BARRIER_RADIUS,
        side::{SIDE_WIDTH, Side, SideSpawnPoint},
    },
    util::hemisphere_deflection,
};

use super::{
    ball::Ball,
    crab::{ai::AI, player::Player},
};

pub const CRAB_WIDTH: f32 = 0.2;
pub const CRAB_DEPTH: f32 = 0.1;
pub const CRAB_START_POSITION: Vec3 = Vec3::new(0.0, 0.05, 0.0);
pub const CRAB_POSITION_X_MAX: f32 =
    (0.5 * SIDE_WIDTH) - BARRIER_RADIUS - (0.5 * CRAB_WIDTH);

pub(super) struct CrabPlugin;

impl Plugin for CrabPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ai::AiPlugin, player::InputPlugin))
            .add_systems(
                OnExit(GameState::StartMenu),
                start_game_with_new_crabs_for_each_side,
            )
            .add_systems(
                Update,
                restrict_crab_movement_to_space_within_its_own_goal
                    .after(PausableSet),
            )
            .add_systems(
                PostUpdate,
                crab_and_ball_collisions.in_set(PausableSet),
            );
    }
}

/// Makes a crab entity that can deflect balls and move sideways inside a goal.
#[derive(Component, Debug, Default)]
pub struct Crab;

fn start_game_with_new_crabs_for_each_side(
    cached_assets: Res<CachedAssets>,
    game_assets: Res<GameAssets>,
    game_modes: GameModes,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    spawn_points_query: Query<(Entity, &Side), With<SideSpawnPoint>>,
) {
    for (spawn_point_entity, side) in &spawn_points_query {
        let crab_config = &game_modes.current().competitors[side];

        commands
            .entity(spawn_point_entity)
            .with_children(|builder| {
                let mut crab = builder.spawn((
                    Crab,
                    *side,
                    Collider,
                    InsertAfterFadeIn::<Movement>::default(),
                    RemoveBeforeFadeOut::<Movement>::default(),
                    RemoveBeforeFadeOut::<Collider>::default(),
                    Fade::new_in(),
                    FadeAnimation::Scale {
                        max_scale: Vec3::new(
                            CRAB_WIDTH, CRAB_DEPTH, CRAB_DEPTH,
                        ),
                        axis_mask: Vec3::ONE,
                    },
                    Heading(Dir3::X),
                    MaxSpeed(crab_config.max_speed),
                    Acceleration(
                        crab_config.max_speed
                            / crab_config.seconds_to_max_speed,
                    ),
                    Mesh3d(cached_assets.crab_mesh.clone()),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(
                            game_assets.image_crab.clone(),
                        ),
                        base_color:
                            Srgba::hex(&crab_config.color).unwrap().into(),
                        ..default()
                    })),
                    Transform::from_matrix(
                        Mat4::from_scale_rotation_translation(
                            Vec3::splat(f32::EPSILON),
                            Quat::IDENTITY,
                            CRAB_START_POSITION,
                        ),
                    ),
                ));

                if crab_config.controller == CrabController::AI {
                    crab.insert(AI);
                } else {
                    crab.insert(Player);
                }
            });

        info!("Crab({side:?}): Spawned");
    }
}

fn restrict_crab_movement_to_space_within_its_own_goal(
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut Transform, &mut Speed, &mut StoppingDistance),
        (With<Crab>, With<Movement>),
    >,
) {
    for (entity, mut transform, mut speed, mut stopping_distance) in &mut query
    {
        // Limit crab movement to the bounds of its own goal.
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

        // Also limit stopping distance to the bounds of the goal.
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
        (Entity, &GlobalTransform, &Heading, &CircleCollider),
        (With<Ball>, With<Collider>, With<Movement>),
    >,
    crabs_query: Query<(&Side, &Transform), (With<Crab>, With<Collider>)>,
) {
    for (ball_entity, ball_transform, ball_heading, ball_collider) in
        &balls_query
    {
        for (side, crab_transform) in &crabs_query {
            // Check that the ball is touching the crab and facing the goal.
            let axis = side.axis();
            let ball_to_side_distance = side.distance_to_ball(ball_transform);
            let ball_side_position = side.get_ball_position(ball_transform);
            let delta = crab_transform.translation.x - ball_side_position;
            let ball_to_crab_distance = delta.abs();

            if ball_to_side_distance > ball_collider.radius + (0.5 * CRAB_DEPTH)
                || ball_to_crab_distance
                    > ball_collider.radius + (0.5 * CRAB_WIDTH)
                || ball_heading.0.dot(axis) <= 0.0
            {
                continue;
            }

            let ball_deflection_direction =
                hemisphere_deflection(delta, CRAB_WIDTH, axis);

            commands
                .entity(ball_entity)
                .insert(Heading(Dir3::new_unchecked(
                    ball_deflection_direction.normalize(),
                )));
            info!("Ball({ball_entity:?}): Collided Crab({side:?})");
            break;
        }
    }
}
