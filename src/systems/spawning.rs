use bevy::prelude::*;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use spew::prelude::*;

use crate::{
    cached_assets::CachedAssets,
    components::{
        goals::{Goal, Side, Wall},
        movement::{
            Acceleration, AccelerationBundle, Heading, MaxSpeed, Speed,
            VelocityBundle,
        },
        paddles::{AiInput, Ball, KeyboardInput, Paddle, Team},
        spawning::{
            Despawning, ForState, Object, Spawning, SpawningAnimation,
            SpawningBundle,
        },
    },
    constants::*,
    global_data::GlobalData,
    screens::GameScreen,
    serialization::{Config, ControlledByConfig, TeamConfig},
};

#[derive(Event)]
struct RemoveGoalOccupantEvent(Entity);

fn remove_goal_occupant(
    mut commands: Commands,
    mut event_reader: EventReader<RemoveGoalOccupantEvent>,
    paddles_and_walls_query: Query<
        (Entity, &Parent),
        (Or<(With<Paddle>, With<Wall>)>, Without<Spawning>),
    >,
) {
    for RemoveGoalOccupantEvent(goal_entity) in event_reader.iter() {
        for (entity, parent) in &paddles_and_walls_query {
            if parent.get() == *goal_entity {
                commands
                    .entity(entity)
                    .remove::<AccelerationBundle>()
                    .insert(Despawning::default());
                break;
            }
        }
    }
}

fn spawn_ball(
    config: Res<Config>,
    cached_assets: Res<CachedAssets>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn a ball that will launch it in a random direction.
    let mut rng = SmallRng::from_entropy();
    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
    let ball = commands
        .spawn((
            Ball,
            SpawningBundle::default(),
            ForState {
                states: vec![GameScreen::Playing, GameScreen::Paused],
            },
            VelocityBundle {
                heading: Heading(Vec3::new(angle.cos(), 0.0, angle.sin())),
                speed: Speed(config.ball_speed),
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
                        BALL_SPAWNER_POSITION,
                    ),
                ),
                ..default()
            },
        ))
        .id();

    info!("Ball({:?}): Spawned", ball);
}

fn spawn_wall_in_goal(
    In(side): In<Side>,
    cached_assets: Res<CachedAssets>,
    mut commands: Commands,
    goals_query: Query<(Entity, &Side), With<Goal>>,
    mut remove_goal_occupant_writer: EventWriter<RemoveGoalOccupantEvent>,
) {
    for (goal_entity, goal_side) in &goals_query {
        if *goal_side != side {
            continue;
        }

        remove_goal_occupant_writer.send(RemoveGoalOccupantEvent(goal_entity));

        // Spawn wall in goal.
        let wall = commands
            .entity(goal_entity)
            .with_children(|parent| {
                parent.spawn((
                    *goal_side,
                    Wall,
                    SpawningBundle {
                        spawning_animation: SpawningAnimation::Scale {
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
        break;
    }
}

fn spawn_paddle_in_goal(
    In(side): In<Side>,
    global_data: Res<GlobalData>,
    config: Res<Config>,
    cached_assets: Res<CachedAssets>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    goals_query: Query<(Entity, &Side), With<Goal>>,
    mut remove_goal_occupant_writer: EventWriter<RemoveGoalOccupantEvent>,
) {
    for (i, (goal_entity, goal_side)) in goals_query.iter().enumerate() {
        if *goal_side != side {
            continue;
        }

        remove_goal_occupant_writer.send(RemoveGoalOccupantEvent(goal_entity));

        // Spawn paddle in goal.
        let goal_config = &config.modes[global_data.mode_index].goals[i];
        let material_handle = cached_assets.paddle_materials[goal_side].clone();
        let paddle = commands
            .entity(goal_entity)
            .with_children(|parent| {
                let mut paddle = parent.spawn((
                    *goal_side,
                    Paddle,
                    SpawningBundle {
                        spawning_animation: SpawningAnimation::Scale {
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
                        max_speed: MaxSpeed(config.paddle_max_speed),
                        acceleration: Acceleration(
                            config.paddle_max_speed
                                / config.paddle_seconds_to_max_speed,
                        ),
                        ..default()
                    },
                    PbrBundle {
                        mesh: cached_assets.paddle_mesh.clone(),
                        material: material_handle.clone(),
                        transform: Transform::from_matrix(
                            Mat4::from_scale_rotation_translation(
                                Vec3::splat(f32::EPSILON),
                                Quat::IDENTITY,
                                GOAL_PADDLE_START_POSITION,
                            ),
                        ),
                        ..default()
                    },
                    if goal_config.team == TeamConfig::Enemies {
                        Team::Enemies
                    } else {
                        Team::Allies
                    },
                ));

                if goal_config.controlled_by == ControlledByConfig::AI {
                    paddle.insert(AiInput);
                } else {
                    paddle.insert(KeyboardInput);
                }

                let material = materials.get_mut(&material_handle).unwrap();
                material.base_color = Color::hex(&goal_config.color).unwrap()
            })
            .id();

        info!("Paddle({:?}): Spawned", paddle);
        break;
    }
}

pub struct SpawningPlugin;

impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RemoveGoalOccupantEvent>()
            .add_systems(Update, remove_goal_occupant.after(SpewSystemSet))
            .add_plugins(SpewPlugin::<Object>::default())
            .add_plugins(SpewPlugin::<Object, Side>::default())
            .add_spawners((
                (Object::Ball, spawn_ball),
                (Object::Wall, spawn_wall_in_goal),
                (Object::Paddle, spawn_paddle_in_goal),
            ));
    }
}