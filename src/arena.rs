use bevy::prelude::*;
use spew::prelude::{SpawnEvent, SpewSystemSet};

use crate::{
    assets::GameAssets,
    ball::{Ball, Collider, BALL_HEIGHT},
    config::{GameConfig, GameMode},
    goal::{
        Barrier, Goal, BARRIER_DIAMETER, BARRIER_HEIGHT, GOAL_HALF_WIDTH,
        GOAL_WIDTH,
    },
    movement::Active,
    object::Object,
    ocean::Ocean,
    side::Side,
    state::AppState,
    swaying_camera::SwayingCamera,
};

pub const ARENA_CENTER_POINT: Vec3 = Vec3::ZERO;
pub const ARENA_BALL_SPAWNER_POSITION: Vec3 = Vec3::new(
    ARENA_CENTER_POINT.x,
    ARENA_CENTER_POINT.y + BALL_HEIGHT,
    ARENA_CENTER_POINT.z,
);

/// Global data related to the play area.
#[derive(Debug, Default, Resource)]
pub struct Arena {
    max_ball_count: u8,
}

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Loading), spawn_level)
            .add_systems(
                OnExit(AppState::StartMenu),
                (initialize_arena_data, give_each_goal_a_new_paddle),
            )
            .add_systems(
                Update,
                spawn_balls_sequentially_as_needed
                    .before(SpewSystemSet)
                    .run_if(in_state(AppState::Playing)),
            );
    }
}

fn initialize_arena_data(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    game_configs: Res<Assets<GameConfig>>,
    game_mode: Res<GameMode>,
) {
    let game_config = game_configs.get(&game_assets.game_config).unwrap();

    commands.insert_resource(Arena {
        max_ball_count: game_config.modes[game_mode.0].max_ball_count,
    })
}

fn give_each_goal_a_new_paddle(
    goals_query: Query<Entity, With<Goal>>,
    mut spawn_in_goal_events: EventWriter<SpawnEvent<Object, Entity>>,
) {
    for entity in &goals_query {
        spawn_in_goal_events
            .send(SpawnEvent::with_data(Object::Paddle, entity));
    }
}

fn spawn_balls_sequentially_as_needed(
    arena: Res<Arena>,
    balls_query: Query<Entity, With<Ball>>,
    inactive_balls_query: Query<Entity, (With<Ball>, Without<Active>)>,
    mut spawn_events: EventWriter<SpawnEvent<Object>>,
) {
    if balls_query.iter().len() < arena.max_ball_count as usize
        && inactive_balls_query.iter().len() < 1
    {
        spawn_events.send(SpawnEvent::new(Object::Ball));
    }
}

fn spawn_level(
    game_assets: Res<GameAssets>,
    game_configs: Res<Assets<GameConfig>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spawn_in_goal_events: EventWriter<SpawnEvent<Object, Entity>>,
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
            speed: game_config.animated_water_speed,
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
                ARENA_CENTER_POINT,
            ),
        ),
        ..default()
    });

    // Goals
    let unit_cube = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let barrier_material = materials.add(Color::hex("750000").unwrap().into());
    const SIDES: [Side; 4] = [Side::Bottom, Side::Right, Side::Top, Side::Left];

    for (i, side) in SIDES.iter().enumerate() {
        // Goals
        let goal = commands
            .spawn((
                *side,
                Goal,
                PbrBundle {
                    transform: Transform::from_rotation(Quat::from_axis_angle(
                        Vec3::Y,
                        std::f32::consts::TAU * (i as f32 / SIDES.len() as f32),
                    ))
                    .mul_transform(Transform::from_xyz(
                        0.0,
                        0.0,
                        GOAL_HALF_WIDTH,
                    )),
                    ..default()
                },
            ))
            .with_children(|parent| {
                // Barrier
                parent.spawn((
                    Barrier,
                    Collider,
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
                                    GOAL_HALF_WIDTH,
                                    0.5 * BARRIER_HEIGHT,
                                    0.0,
                                ),
                            ),
                        ),
                        ..default()
                    },
                ));
            })
            .id();

        // Walls
        spawn_in_goal_events.send(SpawnEvent::with_data(Object::Wall, goal));
    }
}
