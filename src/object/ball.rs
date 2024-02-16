use bevy::prelude::*;
use rand::prelude::*;
use spew::prelude::*;

use crate::{
    common::{
        collider::{Collider, ColliderShapeCircle},
        delayed::{DelayedInsert, DelayedRemove},
        fade::FadeBundle,
        movement::{Heading, Movement, Speed, VelocityBundle},
    },
    game::{
        assets::{CachedAssets, GameMode},
        state::{ForStates, GameState, SelectedGameMode},
    },
};

use super::Object;

/// Marks a ball entity that can collide and score.
#[derive(Component, Debug)]
pub struct Ball;

pub(super) struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_spawner((Object::Ball, spawn_ball_with_position));
    }
}

fn spawn_ball_with_position(
    In(position): In<Vec3>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cached_assets: Res<CachedAssets>,
    game_modes: Res<Assets<GameMode>>,
    game_mode: Res<SelectedGameMode>,
) {
    // Spawn a ball that will launch it in a random direction.
    let game_mode = game_modes.get(&game_mode.0).unwrap();
    let mut rng = SmallRng::from_entropy();
    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
    let (angle_sin, angle_cos) = angle.sin_cos();
    let ball = commands
        .spawn((
            Ball,
            ColliderShapeCircle {
                radius: game_mode.ball_size * 0.5,
            },
            DelayedInsert::<Movement>::default(),
            DelayedInsert::<Collider>::default(),
            DelayedRemove::<Collider>::default(),
            FadeBundle::default(),
            ForStates(vec![GameState::Playing, GameState::Paused]),
            VelocityBundle {
                heading: Heading(Vec3::new(angle_cos, 0.0, angle_sin)),
                speed: Speed(game_mode.ball_speed),
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
                        Vec3::splat(game_mode.ball_size),
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

// TODO: Debug option to directly control single ball's exact position with
// keyboard and see how crabs respond. Can go in goals, triggering a score and
// ball return?
