use bevy::prelude::*;
use rand::prelude::*;
use spew::prelude::*;

use crate::{
    common::{
        collider::{Collider, ColliderShapeCircle},
        fade::{Fade, FadeBundle},
        movement::{Heading, Movement, Speed, VelocityBundle},
    },
    game::{
        assets::{CachedAssets, GameAssets, GameConfig},
        state::{ForStates, GameState},
    },
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
                    add_ball_movement_and_collider_after_fading_in,
                    remove_ball_collider_before_fading_out,
                )
                    .run_if(not(in_state(GameState::Paused))),
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
            ColliderShapeCircle {
                radius: BALL_RADIUS,
            },
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

    info!("Ball({ball:?}): Spawned");
}

fn add_ball_movement_and_collider_after_fading_in(
    mut commands: Commands,
    mut removed: RemovedComponents<Fade>,
    query: Query<Entity, With<Ball>>,
) {
    for entity in removed.read() {
        if query.contains(entity) {
            commands.entity(entity).insert(Movement).insert(Collider);
        }
    }
}

fn remove_ball_collider_before_fading_out(
    mut commands: Commands,
    query: Query<(Entity, &Fade), (With<Ball>, Added<Fade>)>,
) {
    for (entity, fade) in &query {
        if matches!(fade, Fade::Out(_)) {
            commands.entity(entity).remove::<Collider>();
        }
    }
}

// TODO: Debug option to directly control single ball's exact position with
// keyboard and see how crabs respond. Can go in goals, triggering a score and
// ball return?
