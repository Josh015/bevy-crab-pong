use bevy::prelude::*;
use spew::prelude::{SpawnEvent, SpewSystemSet};

use crate::{
    common::{
        collider::{Collider, ColliderShapeCircle},
        movement::Movement,
    },
    game::{
        assets::{GameAssets, GameConfig},
        competitors::GameMode,
        state::GameState,
    },
    object::{
        ball::{Ball, BALL_HEIGHT},
        Object,
    },
};

use super::{
    goal::{Goal, GOAL_WIDTH},
    ocean::Ocean,
    side::{Side, SIDES},
    swaying_camera::SwayingCamera,
};

pub const BEACH_CENTER_POINT: Vec3 = Vec3::ZERO;
pub const BARRIER_DIAMETER: f32 = 0.12;
pub const BARRIER_RADIUS: f32 = 0.5 * BARRIER_DIAMETER;
pub const BARRIER_HEIGHT: f32 = 0.2;

/// Global data related to the play area.
#[derive(Debug, Default, Resource)]
pub struct Beach {
    ball_count: u8,
}

pub(super) struct BeachPlugin;

impl Plugin for BeachPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Loading), spawn_level)
            .add_systems(
                OnExit(GameState::StartMenu),
                (initialize_beach_data, give_each_goal_a_new_crab),
            )
            .add_systems(
                Update,
                spawn_balls_sequentially_as_needed
                    .before(SpewSystemSet)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn spawn_level(
    game_assets: Res<GameAssets>,
    game_configs: Res<Assets<GameConfig>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spawn_on_side_events: EventWriter<SpawnEvent<Object, Side>>,
) {
    let game_config = game_configs.get(&game_assets.game_config).unwrap();

    // Cameras
    commands.spawn((
        SwayingCamera {
            speed: game_config.swaying_camera_speed,
        },
        Camera3dBundle::default(),
    ));

    // Light
    let light_transform = Mat4::from_euler(
        EulerRot::ZYX,
        0.0,
        std::f32::consts::FRAC_PI_4,
        -std::f32::consts::FRAC_PI_4,
    );
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            // shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_matrix(light_transform),
        ..default()
    });

    // Ocean
    commands.spawn((
        Ocean {
            speed: game_config.ocean_scroll_speed,
            ..default()
        },
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: 100.0,
                subdivisions: 1,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("257AFFCC").unwrap(),
                alpha_mode: AlphaMode::Blend,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, -0.01, 0.0),
            ..default()
        },
    ));

    // Beach
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 1.0,
            subdivisions: 1,
        })),
        material: materials.add(Color::hex("C4BD99").unwrap().into()),
        transform: Transform::from_matrix(
            Mat4::from_scale_rotation_translation(
                Vec3::splat(GOAL_WIDTH),
                Quat::IDENTITY,
                BEACH_CENTER_POINT,
            ),
        ),
        ..default()
    });

    // Goals
    let unit_cube = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let barrier_material = materials.add(Color::hex("750000").unwrap().into());

    for (i, side) in SIDES.iter().enumerate() {
        // Goals
        commands
            .spawn((
                Goal,
                *side,
                PbrBundle {
                    transform: Transform::from_rotation(Quat::from_axis_angle(
                        Vec3::Y,
                        std::f32::consts::TAU * (i as f32 / SIDES.len() as f32),
                    ))
                    .mul_transform(Transform::from_xyz(
                        0.0,
                        0.0,
                        0.5 * GOAL_WIDTH,
                    )),
                    ..default()
                },
            ))
            .with_children(|builder| {
                // Barrier
                builder.spawn((
                    Collider,
                    ColliderShapeCircle {
                        radius: BARRIER_RADIUS,
                    },
                    PbrBundle {
                        mesh: unit_cube.clone(),
                        material: barrier_material.clone(),
                        transform: Transform::from_matrix(
                            Mat4::from_scale_rotation_translation(
                                Vec3::new(
                                    BARRIER_DIAMETER,
                                    BARRIER_HEIGHT,
                                    BARRIER_DIAMETER,
                                ),
                                Quat::IDENTITY,
                                Vec3::new(
                                    0.5 * GOAL_WIDTH,
                                    0.5 * BARRIER_HEIGHT,
                                    0.0,
                                ),
                            ),
                        ),
                        ..default()
                    },
                ));
            });

        // Walls
        spawn_on_side_events.send(SpawnEvent::with_data(Object::Wall, *side));
    }
}

fn initialize_beach_data(
    mut commands: Commands,
    game_mode: Res<GameMode>,
    game_assets: Res<GameAssets>,
    game_configs: Res<Assets<GameConfig>>,
) {
    let game_config = game_configs.get(&game_assets.game_config).unwrap();

    commands.insert_resource(Beach {
        ball_count: u8::from(game_config.modes[game_mode.0].ball_count),
    });
}

fn give_each_goal_a_new_crab(
    mut spawn_on_side_events: EventWriter<SpawnEvent<Object, Side>>,
) {
    for side in SIDES {
        spawn_on_side_events.send(SpawnEvent::with_data(Object::Crab, side));
    }
}

fn spawn_balls_sequentially_as_needed(
    beach: Res<Beach>,
    balls_query: Query<Entity, With<Ball>>,
    non_moving_balls_query: Query<Entity, (With<Ball>, Without<Movement>)>,
    mut spawn_with_position_events: EventWriter<SpawnEvent<Object, Vec3>>,
) {
    // Make balls spawn, fade in, and then launch one at a time.
    if balls_query.iter().len() >= beach.ball_count as usize
        || non_moving_balls_query.iter().len() >= 1
    {
        return;
    }

    spawn_with_position_events.send(SpawnEvent::with_data(
        Object::Ball,
        Vec3::new(
            BEACH_CENTER_POINT.x,
            BEACH_CENTER_POINT.y + BALL_HEIGHT,
            BEACH_CENTER_POINT.z,
        ),
    ));
}
