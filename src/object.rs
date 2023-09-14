use bevy::prelude::*;
use spew::prelude::*;

use crate::{
    assets::{CachedAssets, GameAssets, GameConfig, PlayerConfig},
    beach::Beach,
    collider::Collider,
    crab::{AiPlayer, Crab, KeyboardPlayer, CRAB_SCALE, CRAB_START_POSITION},
    fade::{Fade, FadeAnimation, FadeBundle, FADE_DURATION_IN_SECONDS},
    game::GameMode,
    goal::Goal,
    movement::{
        Acceleration, AccelerationBundle, Heading, MaxSpeed, VelocityBundle,
    },
    side::Side,
    wall::{Wall, WALL_HEIGHT, WALL_SCALE},
};

/// Objects that can be spawned via Spew.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Object {
    Wall,
    Crab,
}

pub struct ObjectPlugin;

impl Plugin for ObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_spawners((
            (Object::Wall, spawn_wall_on_side),
            (Object::Crab, spawn_crab_on_side),
        ))
        .add_plugins(SpewPlugin::<Object>::default())
        .add_plugins(SpewPlugin::<Object, Side>::default());
    }
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
                        CRAB_START_POSITION,
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
