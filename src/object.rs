use bevy::prelude::*;
use rand::prelude::*;
use spew::prelude::*;

use crate::{
    assets::{CachedAssets, GameAssets, GameConfig, Player},
    ball::{Ball, BALL_DIAMETER},
    beach::Beach,
    collider::Collider,
    crab::{Crab, CRAB_DEPTH, CRAB_START_POSITION, CRAB_WIDTH},
    fade::{Fade, FadeAnimation, FadeBundle, FADE_DURATION_IN_SECONDS},
    game::GameMode,
    goal::{Goal, GOAL_WIDTH},
    movement::{
        Acceleration, AccelerationBundle, Heading, MaxSpeed, Speed,
        VelocityBundle,
    },
    player::{ai::PlayerAi, input::PlayerInput},
    side::Side,
    state::{ForStates, GameState},
    wall::{Wall, WALL_DIAMETER, WALL_HEIGHT},
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
            (Object::Ball, spawn_ball_with_position),
            (Object::Wall, spawn_wall_on_side),
            (Object::Crab, spawn_crab_on_side),
        ))
        .add_plugins((
            SpewPlugin::<Object, Vec3>::default(),
            SpewPlugin::<Object, Side>::default(),
        ));
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
            Collider,
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

fn spawn_wall_on_side(
    In(side): In<Side>,
    cached_assets: Res<CachedAssets>,
    mut commands: Commands,
    beach: Option<Res<Beach>>,
    goals_query: Query<(Entity, &Side), With<Goal>>,
) {
    let (goal_entity, _) = goals_query
        .iter()
        .find(|(_, goal_side)| **goal_side == side)
        .unwrap();

    commands.entity(goal_entity).with_children(|builder| {
        builder.spawn((
            Wall,
            Collider,
            side,
            FadeBundle {
                fade_animation: FadeAnimation::Scale {
                    max_scale: Vec3::new(
                        GOAL_WIDTH,
                        WALL_DIAMETER,
                        WALL_DIAMETER,
                    ),
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

    info!("Wall({:?}): Spawned", side);
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
