use bevy::prelude::*;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use spew::prelude::*;

use crate::{
    assets::{CachedAssets, GameAssets, GameConfig, PlayerConfig},
    ball::{Ball, BALL_DIAMETER},
    beach::{Beach, BEACH_BALL_SPAWNER_POSITION},
    collider::Collider,
    crab::{AiPlayer, Crab, KeyboardPlayer, CRAB_SCALE},
    fade::{Fade, FadeAnimation, FadeBundle, FADE_DURATION_IN_SECONDS},
    game::GameMode,
    goal::{Goal, GOAL_CRAB_START_POSITION},
    movement::{
        Acceleration, AccelerationBundle, Heading, MaxSpeed, Speed,
        VelocityBundle,
    },
    side::Side,
    state::{ForStates, GameState},
    wall::{Wall, WALL_HEIGHT, WALL_SCALE},
};

/// Objects that can be spawned via Spew.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Object {
    Ball,
    Wall,
    Crab,
}

pub struct ObjectPlugin;

impl Plugin for ObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_spawners((
            (Object::Ball, spawn_ball),
            (Object::Wall, spawn_wall_on_side),
            (Object::Crab, spawn_crab_on_side),
        ))
        .add_plugins(SpewPlugin::<Object>::default())
        .add_plugins(SpewPlugin::<Object, Side>::default());
    }
}

fn spawn_ball(
    game_assets: Res<GameAssets>,
    game_configs: Res<Assets<GameConfig>>,
    cached_assets: Res<CachedAssets>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn a ball that will launch it in a random direction.
    let game_config = game_configs.get(&game_assets.game_config).unwrap();
    let mut rng = SmallRng::from_entropy();
    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
    let ball = commands
        .spawn((
            Ball,
            Collider,
            FadeBundle::default(),
            ForStates(vec![GameState::Playing, GameState::Paused]),
            VelocityBundle {
                heading: Heading(Vec3::new(angle.cos(), 0.0, angle.sin())),
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
                        BEACH_BALL_SPAWNER_POSITION,
                    ),
                ),
                ..default()
            },
        ))
        .id();

    info!("Ball({:?}): Spawned", ball);
}

fn spawn_wall_on_side(
    In(goal_side): In<Side>,
    cached_assets: Res<CachedAssets>,
    mut commands: Commands,
    beach: Option<Res<Beach>>,
    goals_query: Query<(Entity, &Side), With<Goal>>,
) {
    let Some((goal_entity, _)) =
        goals_query.iter().find(|(_, side)| **side == goal_side)
    else {
        return;
    };

    // Spawn wall in goal.
    commands.entity(goal_entity).with_children(|builder| {
        builder.spawn((
            Wall,
            Collider,
            goal_side,
            FadeBundle {
                fade_animation: FadeAnimation::Scale {
                    max_scale: WALL_SCALE,
                    axis_mask: Vec3::new(0.0, 1.0, 1.0),
                },
                fade: Fade::In(Timer::from_seconds(
                    if beach.is_none() {
                        0.0
                    } else {
                        FADE_DURATION_IN_SECONDS
                    },
                    TimerMode::Once,
                )),
            },
            PbrBundle {
                mesh: cached_assets.wall_mesh.clone(),
                material: cached_assets.wall_material.clone(),
                transform: Transform::from_matrix(
                    Mat4::from_scale_rotation_translation(
                        Vec3::splat(f32::EPSILON),
                        Quat::IDENTITY,
                        Vec3::new(0.0, WALL_HEIGHT, 0.0),
                    ),
                ),
                ..default()
            },
        ));
    });

    info!("Wall({:?}): Spawned", goal_side);
}

fn spawn_crab_on_side(
    In(goal_side): In<Side>,
    cached_assets: Res<CachedAssets>,
    game_mode: Res<GameMode>,
    game_assets: Res<GameAssets>,
    game_configs: Res<Assets<GameConfig>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    goals_query: Query<(Entity, &Side), With<Goal>>,
) {
    let Some((goal_entity, _)) =
        goals_query.iter().find(|(_, side)| **side == goal_side)
    else {
        return;
    };

    let game_config = game_configs.get(&game_assets.game_config).unwrap();
    let crab_config = &game_config.modes[game_mode.0].competitors[&goal_side];
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(game_assets.image_crab.clone()),
        base_color: Color::hex(&crab_config.color).unwrap(),
        ..default()
    });

    commands.entity(goal_entity).with_children(|builder| {
        let mut crab = builder.spawn((
            Crab,
            Collider,
            goal_side,
            FadeBundle {
                fade_animation: FadeAnimation::Scale {
                    max_scale: CRAB_SCALE,
                    axis_mask: Vec3::ONE,
                },
                ..default()
            },
            AccelerationBundle {
                velocity: VelocityBundle {
                    heading: Heading(Vec3::X),
                    ..default()
                },
                max_speed: MaxSpeed(game_config.crab_max_speed),
                acceleration: Acceleration(
                    game_config.crab_max_speed
                        / game_config.crab_seconds_to_max_speed,
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
                        GOAL_CRAB_START_POSITION,
                    ),
                ),

                ..default()
            },
        ));

        if crab_config.player == PlayerConfig::AI {
            crab.insert(AiPlayer);
        } else {
            crab.insert(KeyboardPlayer);
        }
    });

    info!("Crab({:?}): Spawned", goal_side);
}
