pub mod ball;
pub mod crab;
pub mod wall;

use bevy::prelude::*;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use spew::prelude::*;

use crate::{
    assets::{CachedAssets, GameAssets},
    collider::Collider,
    config::{GameConfig, GameMode, PlayerConfig},
    fade::{FadeAnimation, FadeBundle},
    level::{
        beach::BEACH_BALL_SPAWNER_POSITION,
        goal::{Goal, GOAL_CRAB_START_POSITION},
        side::Side,
    },
    movement::{
        Acceleration, AccelerationBundle, Heading, MaxSpeed, Speed,
        VelocityBundle,
    },
    object::{
        ball::{Ball, BALL_DIAMETER},
        crab::{AiPlayer, Crab, HitPoints, KeyboardPlayer, CRAB_SCALE},
        wall::{Wall, WALL_HEIGHT, WALL_SCALE},
    },
    state::{AppState, ForStates},
    team::Team,
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
            (Object::Wall, spawn_wall_in_goal),
            (Object::Crab, spawn_crab_in_goal),
        ))
        .add_plugins((
            SpewPlugin::<Object>::default(),
            SpewPlugin::<Object, Entity>::default(),
        ))
        .add_plugins((
            ball::BallPlugin,
            crab::CrabPlugin,
            wall::WallPlugin,
        ));
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
            ForStates([AppState::Playing, AppState::Paused]),
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

fn spawn_wall_in_goal(
    In(goal_entity): In<Entity>,
    cached_assets: Res<CachedAssets>,
    mut commands: Commands,
    goals_query: Query<&Side, With<Goal>>,
) {
    let Ok(goal_side) = goals_query.get(goal_entity) else {
        return;
    };

    // Spawn wall in goal.
    let wall = commands
        .entity(goal_entity)
        .with_children(|parent| {
            parent.spawn((
                Wall,
                Collider,
                *goal_side,
                FadeBundle {
                    fade_animation: FadeAnimation::Scale {
                        max_scale: WALL_SCALE,
                        axis_mask: Vec3::new(0.0, 1.0, 1.0),
                    },
                    ..default()
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
        })
        .id();

    info!("Wall({:?}): Spawned", wall);
}

fn spawn_crab_in_goal(
    In(goal_entity): In<Entity>,
    game_mode: Res<GameMode>,
    cached_assets: Res<CachedAssets>,
    game_assets: Res<GameAssets>,
    game_configs: Res<Assets<GameConfig>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    goals_query: Query<&Side, With<Goal>>,
) {
    let Ok(goal_side) = goals_query.get(goal_entity) else {
        return;
    };

    let game_config = game_configs.get(&game_assets.game_config).unwrap();
    let crab_config = &game_config.modes[game_mode.0].crabs[goal_side];
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(game_assets.image_crab.clone()),
        base_color: Color::hex(&crab_config.color).unwrap(),
        ..default()
    });

    let crab = commands
        .entity(goal_entity)
        .with_children(|parent| {
            let mut crab = parent.spawn((
                Crab,
                Collider,
                *goal_side,
                Team(crab_config.team),
                HitPoints(crab_config.hit_points),
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
        })
        .id();

    info!("Crab({:?}): Spawned", crab);
}
