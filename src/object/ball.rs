use bevy::prelude::*;
use rand::prelude::*;
use spew::prelude::*;

use crate::{
    common::{
        collider::{Collider, ColliderSet},
        fade::{Fade, FadeBundle},
        movement::{Heading, Movement, Speed, VelocityBundle},
    },
    game::{
        assets::{CachedAssets, GameAssets, GameConfig},
        state::{ForStates, GameState},
    },
    util::reflect,
};

use super::Object;

pub const BALL_DIAMETER: f32 = 0.08;
pub const BALL_HEIGHT: f32 = 0.05;
pub const BALL_RADIUS: f32 = 0.5 * BALL_DIAMETER;

/// Marks a ball entity that can collide and score.
#[derive(Component, Debug)]
pub struct Ball;

pub(super) struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_spawner((Object::Ball, spawn_ball_with_position))
            .add_systems(
                Update,
                (
                    add_ball_movement_and_collision_after_fading_in,
                    remove_ball_collision_before_fading_out,
                )
                    .run_if(not(in_state(GameState::Paused))),
            )
            .add_systems(
                PostUpdate,
                ball_and_ball_collisions.in_set(ColliderSet),
            );
    }
}

fn spawn_ball_with_position(
    In(position): In<Vec3>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cached_assets: Res<CachedAssets>,
    game_assets: Res<GameAssets>,
    game_configs: Res<Assets<GameConfig>>,
) {
    // Spawn a ball that will launch it in a random direction.
    let game_config = game_configs.get(&game_assets.game_config).unwrap();
    let mut rng = SmallRng::from_entropy();
    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
    let (angle_sin, angle_cos) = angle.sin_cos();
    let ball = commands
        .spawn((
            Ball,
            FadeBundle::default(),
            ForStates(vec![GameState::Playing, GameState::Paused]),
            VelocityBundle {
                heading: Heading(Vec3::new(angle_cos, 0.0, angle_sin)),
                speed: Speed(game_config.ball_speed),
            },
            PbrBundle {
                mesh: cached_assets.ball_mesh.clone(),
                material: materials.add(StandardMaterial {
                    alpha_mode: AlphaMode::Blend,
                    base_color: Color::rgba(1.0, 1.0, 1.0, 0.0),
                    ..default()
                }),
                transform: Transform::from_matrix(
                    Mat4::from_scale_rotation_translation(
                        Vec3::splat(BALL_DIAMETER),
                        Quat::IDENTITY,
                        position,
                    ),
                ),
                ..default()
            },
        ))
        .id();

    info!("Ball({:?}): Spawned", ball);
}

fn add_ball_movement_and_collision_after_fading_in(
    mut commands: Commands,
    mut removed: RemovedComponents<Fade>,
    query: Query<Entity, With<Ball>>,
) {
    for entity in removed.iter() {
        if query.contains(entity) {
            commands.entity(entity).insert(Movement).insert(Collider);
        }
    }
}

fn remove_ball_collision_before_fading_out(
    mut commands: Commands,
    query: Query<(Entity, &Fade), (With<Ball>, Added<Fade>)>,
) {
    for (entity, fade) in &query {
        if matches!(fade, Fade::Out(_)) {
            commands.entity(entity).remove::<Collider>();
        }
    }
}

fn ball_and_ball_collisions(
    mut commands: Commands,
    balls_query: Query<
        (Entity, &GlobalTransform, &Heading),
        (With<Ball>, With<Movement>, With<Collider>),
    >,
) {
    for [(entity1, transform1, heading1), (entity2, transform2, heading2)] in
        balls_query.iter_combinations()
    {
        // Check that both balls are close enough to touch.
        let delta = transform2.translation() - transform1.translation();

        if delta.length() > BALL_RADIUS + BALL_RADIUS {
            continue;
        }

        // Deflect both balls away from each other.
        let axis1 = delta.normalize();
        let axis2 = -axis1;
        let is_b1_facing_b2 = heading1.0.dot(axis1) > 0.0;
        let is_b2_facing_b1 = heading2.0.dot(axis2) > 0.0;

        if is_b1_facing_b2 {
            commands
                .entity(entity1)
                .insert(Heading(reflect(heading1.0, axis1).normalize()));
        } else if is_b2_facing_b1 {
            commands.entity(entity1).insert(Heading(axis2));
        }

        if is_b2_facing_b1 {
            commands
                .entity(entity2)
                .insert(Heading(reflect(heading2.0, axis2).normalize()));
        } else if is_b1_facing_b2 {
            commands.entity(entity2).insert(Heading(axis1));
        }

        info!("Ball({:?}): Collided Ball({:?})", entity1, entity2);
    }
}

// TODO: Debug option to directly control single ball's exact position with
// keyboard and see how crabs respond. Can go in goals, triggering a score and
// ball return?
