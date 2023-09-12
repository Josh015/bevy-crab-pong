use bevy::prelude::*;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use spew::prelude::*;

use crate::{
    arena::ARENA_BALL_SPAWNER_POSITION,
    assets::{CachedAssets, GameAssets},
    ball::{Ball, BALL_DIAMETER},
    config::{GameConfig, GameMode, PlayerConfig},
    goal::{Goal, Wall, GOAL_PADDLE_START_POSITION, WALL_HEIGHT, WALL_SCALE},
    movement::{
        Acceleration, AccelerationBundle, Heading, MaxSpeed, Speed,
        VelocityBundle,
    },
    paddle::{AiPlayer, HitPoints, KeyboardPlayer, Paddle, PADDLE_SCALE},
    side::Side,
    spawning::{DespawningBundle, SpawnAnimation, SpawnAnimationBundle},
    state::{AppState, ForStates},
    team::Team,
};

/// Objects that can be spawned via Spew.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Object {
    Ball,
    Wall,
    Paddle,
}

pub struct ObjectPlugin;

impl Plugin for ObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SpewPlugin::<Object>::default())
            .add_plugins(SpewPlugin::<Object, Entity>::default())
            .add_spawners((
                (Object::Ball, spawn_ball),
                (Object::Wall, spawn_wall_in_goal),
                (Object::Paddle, spawn_paddle_in_goal),
            ))
            .add_systems(
                Update,
                remove_previous_goal_occupant.after(SpewSystemSet),
            );
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
            SpawnAnimationBundle::default(),
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
                        ARENA_BALL_SPAWNER_POSITION,
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
                *goal_side,
                SpawnAnimationBundle {
                    spawn_animation: SpawnAnimation::Scale {
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

fn spawn_paddle_in_goal(
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
    let paddle_config = &game_config.modes[game_mode.0].paddles[goal_side];
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(game_assets.image_paddle.clone()),
        base_color: Color::hex(&paddle_config.color).unwrap(),
        ..default()
    });

    let paddle = commands
        .entity(goal_entity)
        .with_children(|parent| {
            let mut paddle = parent.spawn((
                Paddle,
                *goal_side,
                Team(paddle_config.team),
                HitPoints(paddle_config.hit_points),
                SpawnAnimationBundle {
                    spawn_animation: SpawnAnimation::Scale {
                        max_scale: PADDLE_SCALE,
                        axis_mask: Vec3::ONE,
                    },
                    ..default()
                },
                AccelerationBundle {
                    velocity: VelocityBundle {
                        heading: Heading(Vec3::X),
                        ..default()
                    },
                    max_speed: MaxSpeed(game_config.paddle_max_speed),
                    acceleration: Acceleration(
                        game_config.paddle_max_speed
                            / game_config.paddle_seconds_to_max_speed,
                    ),
                    ..default()
                },
                PbrBundle {
                    mesh: cached_assets.paddle_mesh.clone(),
                    material: material_handle,
                    transform: Transform::from_matrix(
                        Mat4::from_scale_rotation_translation(
                            Vec3::splat(f32::EPSILON),
                            Quat::IDENTITY,
                            GOAL_PADDLE_START_POSITION,
                        ),
                    ),

                    ..default()
                },
            ));

            if paddle_config.player == PlayerConfig::AI {
                paddle.insert(AiPlayer);
            } else {
                paddle.insert(KeyboardPlayer);
            }
        })
        .id();

    info!("Paddle({:?}): Spawned", paddle);
}

fn remove_previous_goal_occupant(
    mut commands: Commands,
    new_query: Query<(Entity, &Parent), Or<(Added<Paddle>, Added<Wall>)>>,
    old_query: Query<(Entity, &Parent), Or<(With<Paddle>, With<Wall>)>>,
) {
    for (new_entity, new_parent) in &new_query {
        for (old_entity, old_parent) in &old_query {
            if old_parent == new_parent && old_entity != new_entity {
                commands
                    .entity(old_entity)
                    .remove::<AccelerationBundle>()
                    .insert(DespawningBundle::default());
                break;
            }
        }
    }
}
