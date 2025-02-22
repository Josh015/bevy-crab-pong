use bevy::prelude::*;
use rand::prelude::*;

use crate::{
    common::{
        collider::{CircleCollider, Collider},
        fade::{FadeBundle, InsertAfterFadeIn, RemoveBeforeFadeOut},
        movement::{Heading, Movement, Speed, VelocityBundle},
    },
    game::{
        assets::CachedAssets,
        modes::GameModes,
        state::{ForStates, GameState},
    },
};

pub(super) struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) { app.observe(spawn_ball_with_position); }
}

#[derive(Event)]
pub struct SpawnBall(pub Vec3);

/// Marks a ball entity that can collide and score.
#[derive(Component, Debug)]
pub struct Ball;

fn spawn_ball_with_position(
    trigger: Trigger<SpawnBall>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cached_assets: Res<CachedAssets>,
    game_modes: GameModes,
) {
    // Spawn a ball that will launch it in a random direction.
    let position = trigger.event().0;
    let game_mode = game_modes.current();
    let mut rng = SmallRng::from_os_rng();
    let angle = rng.random_range(0.0..std::f32::consts::TAU);
    let (angle_sin, angle_cos) = angle.sin_cos();
    let ball = commands
        .spawn((
            Ball,
            CircleCollider {
                radius: game_mode.ball_size * 0.5,
            },
            InsertAfterFadeIn::<Movement>::default(),
            InsertAfterFadeIn::<Collider>::default(),
            RemoveBeforeFadeOut::<Collider>::default(),
            FadeBundle::default(),
            ForStates(vec![GameState::Playing, GameState::Paused]),
            VelocityBundle {
                heading: Heading(Dir3::new_unchecked(Vec3::new(
                    angle_cos, 0.0, angle_sin,
                ))),
                speed: Speed(game_mode.ball_speed),
            },
            PbrBundle {
                mesh: cached_assets.ball_mesh.clone(),
                material: materials.add(StandardMaterial {
                    alpha_mode: AlphaMode::Blend,
                    base_color: Color::srgba(1.0, 1.0, 1.0, 0.0),
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
