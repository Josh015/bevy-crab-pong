use bevy::prelude::*;
use rand::prelude::*;

use crate::{
    common::{
        collider::{CircleCollider, Collider},
        fade::{Fade, InsertAfterFadeIn, RemoveBeforeFadeOut},
        movement::{Heading, Movement, Speed},
    },
    game::{
        assets::CachedAssets,
        modes::GameModes,
        state::{ForStates, GameState, PlayableSet},
    },
};

pub(super) struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn_balls_sequentially_up_to_max_count.in_set(PlayableSet),
        );
    }
}

pub const BALL_HEIGHT_FROM_GROUND: f32 = 0.05;

/// Marks a ball entity that can collide and score.
#[derive(Component, Debug)]
pub struct Ball;

/// Object that will spawn balls from its center-point.
#[derive(Component, Debug, Default)]
#[require(Transform)]
pub struct BallSpawner;

fn spawn_balls_sequentially_up_to_max_count(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cached_assets: Res<CachedAssets>,
    game_modes: GameModes,
    non_moving_balls_query: Query<Entity, (With<Ball>, Without<Movement>)>,
    balls_query: Query<Entity, With<Ball>>,
    spawner_query: Query<&Transform, With<BallSpawner>>,
) {
    // Wait for previously spawned ball to finish appearing.
    if non_moving_balls_query.iter().len() >= 1 {
        return;
    }

    // Spawn balls up to max ball count.
    let game_mode = game_modes.current();
    let ball_count: u8 = game_mode.ball_count.into();

    if balls_query.iter().len() >= ball_count as usize {
        return;
    }

    // Spawn a ball in a random direction from the center of the spawner.
    let mut rng = SmallRng::from_os_rng();
    let angle = rng.random_range(0.0..std::f32::consts::TAU);
    let (angle_sin, angle_cos) = angle.sin_cos();
    let transform = spawner_query.single();
    let mut position = transform.translation.clone();

    position.y += BALL_HEIGHT_FROM_GROUND;

    let ball = commands
        .spawn((
            Ball,
            CircleCollider {
                radius: game_mode.ball_size * 0.5,
            },
            Fade::new_in(),
            InsertAfterFadeIn::<Movement>::default(),
            InsertAfterFadeIn::<Collider>::default(),
            RemoveBeforeFadeOut::<Collider>::default(),
            ForStates(vec![GameState::Playing, GameState::Paused]),
            Heading(Dir3::new_unchecked(Vec3::new(angle_cos, 0.0, angle_sin))),
            Speed(game_mode.ball_speed),
            Mesh3d(cached_assets.ball_mesh.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                alpha_mode: AlphaMode::Blend,
                base_color: Color::srgba(1.0, 1.0, 1.0, 0.0),
                ..default()
            })),
            Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::splat(game_mode.ball_size),
                Quat::IDENTITY,
                position,
            )),
        ))
        .id();

    info!("Ball({ball:?}): Spawned");
}

// TODO: Debug option to directly control single ball's exact position with
// keyboard and see how crabs respond. Can go in goals, triggering a score and
// ball return?
