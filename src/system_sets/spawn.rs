use bevy::prelude::*;
use spew::prelude::*;

use crate::{
    cached_assets::CachedAssets,
    components::{
        balls::Collider,
        fading::{Fade, FadeAnimation, FadeBundle},
        goals::{Goal, Side, Wall},
        movement::{
            Acceleration, AccelerationBundle, Heading, MaxSpeed, VelocityBundle,
        },
        paddles::{AiInput, KeyboardInput, Paddle, Team},
    },
    constants::*,
    events::FadeOutEntityEvent,
    global_data::GlobalData,
    serialization::{Config, ControlledByConfig, TeamConfig},
};

#[derive(Debug, Eq, PartialEq)]
pub enum Spawn {
    Paddle,
    Wall,
}

fn spawn_wall_in_goal(
    In(side): In<Side>,
    cached_assets: Res<CachedAssets>,
    mut commands: Commands,
    goals_query: Query<(Entity, &Side), With<Goal>>,
    paddles_query: Query<(Entity, &Parent), With<Paddle>>,
    mut fade_out_entity_events: EventWriter<FadeOutEntityEvent>,
) {
    for (goal_entity, goal_side) in goals_query.iter() {
        if *goal_side != side {
            continue;
        }

        // Despawn paddle in goal.
        for (paddle_entity, parent) in &paddles_query {
            if parent.get() != goal_entity {
                continue;
            }

            commands
                .entity(paddle_entity)
                .remove::<(Collider, AccelerationBundle)>();
            fade_out_entity_events.send(FadeOutEntityEvent(paddle_entity));
            break;
        }

        // Spawn wall in goal.
        commands.entity(goal_entity).with_children(|parent| {
            parent.spawn((
                *goal_side,
                Wall,
                Collider,
                FadeBundle {
                    fade_animation: FadeAnimation::Scale {
                        max_scale: WALL_SCALE,
                        axis_mask: Vec3::new(0.0, 1.0, 1.0),
                    },
                    fade: Fade::In(0.0),
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
    walls_query: Query<(Entity, &Parent), With<Wall>>,
    goals_query: Query<(Entity, &Side), With<Goal>>,
    mut fade_out_entity_events: EventWriter<FadeOutEntityEvent>,
) {
    for (i, (goal_entity, goal_side)) in goals_query.iter().enumerate() {
        if *goal_side != side {
            continue;
        }

        // Despawn wall in goal.
        for (wall_entity, parent) in &walls_query {
            if parent.get() != goal_entity {
                continue;
            }

            commands.entity(wall_entity).remove::<Collider>();
            fade_out_entity_events.send(FadeOutEntityEvent(wall_entity));
            break;
        }

        // Spawn paddle in goal.
        let goal_config = &config.modes[global_data.mode_index].goals[i];
        let material_handle = cached_assets.paddle_materials[goal_side].clone();

        commands.entity(goal_entity).with_children(|parent| {
            let mut paddle = parent.spawn((
                *goal_side,
                Paddle,
                Collider,
                FadeBundle {
                    fade_animation: FadeAnimation::Scale {
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
        });
        break;
    }
}

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SpewPlugin::<Spawn, Side>::default())
            .add_spawners((
                (Spawn::Paddle, spawn_paddle_in_goal),
                (Spawn::Wall, spawn_wall_in_goal),
            ));
    }
}
