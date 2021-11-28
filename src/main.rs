mod files;

use std::collections::HashMap;

use bevy::{ecs::prelude::*, prelude::*};
use serde::Deserialize;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Playing,
    GameOver,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GoalLocation {
    Top,
    Right,
    Bottom,
    Left,
}

enum CrabMovementDirection {
    Stop,
    Left,
    Right,
}

#[derive(/* Component, */ Default)]
struct SwayingCamera {
    angle: f32,
}

// #[derive(Component)]
struct Score {
    goal_location: GoalLocation,
}

#[derive(/* Component, */ Default)]
struct AnimatedWater {
    scroll: f32,
}

// #[derive(Component)]
struct Crab {
    goal_location: GoalLocation,
    direction: CrabMovementDirection,
    /* TODO: Maybe store a Vec2 'mask' for handling ball collision axis in a
     * generic way? TODO: How to handle zero score shrinking effect? */
}

// #[derive(Component)]
struct Ball {
    // active: bool,
// opacity: f32,
}

// #[derive(Component)]
struct Pole {
    goal_location: GoalLocation,
    // is_active: bool,
}

// #[derive(Component)]
struct Movable {}

// #[derive(Component)]
enum Collider {
    Crab,
    Ball,
    Pole,
    Barrier,
}

#[derive(Debug, Deserialize)]
struct GameConfig {
    title: String,
    width: u32,
    height: u32,
    swaying_camera_speed: f32,
    animated_water_speed: f32,
    /* startingScore: u8, //20,
     * crabSpeed: f32,    // 2.2,
     * ballSpeed: f32,    // ?? */
}

struct Game {
    scores: HashMap<GoalLocation, u32>,
    player_goal_location: GoalLocation,
}

fn main() {
    let config: GameConfig =
        files::load_config_from_file("assets/config/game.ron");

    App::new()
        .insert_resource(WindowDescriptor {
            title: config.title.clone(),
            width: config.width as f32,
            height: config.height as f32,
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.7, 0.9, 1.0)))
        .add_plugins(DefaultPlugins)
        .insert_resource(config)
        .insert_resource(Game {
            scores: HashMap::from([
                (GoalLocation::Top, 20),
                (GoalLocation::Right, 20),
                (GoalLocation::Bottom, 20),
                (GoalLocation::Left, 20),
            ]),
            player_goal_location: GoalLocation::Bottom,
        })
        .add_startup_system(setup_level)
        .add_startup_system(setup_playable_entities)
        .add_system(swaying_camera_system)
        .add_system(animated_water_system)
        .add_system(crab_score_system)
        .add_system(crab_movement_system)
        .add_system(player_crab_control_system)
        .add_system(ai_crab_control_system)
        .add_system(ball_collision_system)
        .add_system(ball_movement_system)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn setup_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // Camera
    commands
        .spawn_bundle(PerspectiveCameraBundle::default())
        .insert(SwayingCamera::default());

    let unit_plane = meshes.add(Mesh::from(shape::Plane { size: 1.0 }));

    // Ocean
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
            material: materials.add(Color::hex("257AFF").unwrap().into()),
            transform: Transform::from_xyz(0.0, -0.01, 0.0),
            ..Default::default()
        })
        .insert(AnimatedWater::default());

    // Sand
    commands.spawn_bundle(PbrBundle {
        mesh: unit_plane.clone(),
        material: materials.add(Color::hex("C4BD99").unwrap().into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });

    let unit_cube = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));

    // Barriers
    let barrier_material = materials.add(Color::hex("750000").unwrap().into());
    let barrier_height = 0.1;
    let barrier_scale = Vec3::splat(0.20);

    commands
        .spawn_bundle(PbrBundle {
            mesh: unit_cube.clone(),
            material: barrier_material.clone(),
            transform: Transform::from_matrix(
                Mat4::from_scale_rotation_translation(
                    barrier_scale,
                    Quat::IDENTITY,
                    Vec3::new(-0.5, barrier_height, -0.5),
                ),
            ),
            ..Default::default()
        })
        .insert(Collider::Barrier);

    commands
        .spawn_bundle(PbrBundle {
            mesh: unit_cube.clone(),
            material: barrier_material.clone(),
            transform: Transform::from_matrix(
                Mat4::from_scale_rotation_translation(
                    barrier_scale,
                    Quat::IDENTITY,
                    Vec3::new(0.5, barrier_height, -0.5),
                ),
            ),
            ..Default::default()
        })
        .insert(Collider::Barrier);

    commands
        .spawn_bundle(PbrBundle {
            mesh: unit_cube.clone(),
            material: barrier_material.clone(),
            transform: Transform::from_matrix(
                Mat4::from_scale_rotation_translation(
                    barrier_scale,
                    Quat::IDENTITY,
                    Vec3::new(0.5, barrier_height, 0.5),
                ),
            ),
            ..Default::default()
        })
        .insert(Collider::Barrier);

    commands
        .spawn_bundle(PbrBundle {
            mesh: unit_cube.clone(),
            material: barrier_material.clone(),
            transform: Transform::from_matrix(
                Mat4::from_scale_rotation_translation(
                    barrier_scale,
                    Quat::IDENTITY,
                    Vec3::new(-0.5, barrier_height, 0.5),
                ),
            ),
            ..Default::default()
        })
        .insert(Collider::Barrier);

    // Poles
    let pole_material = materials.add(Color::hex("00A400").unwrap().into());
    let pole_height = 0.1;
    let pole_radius = 0.05;
    let pole_scale_x = Vec3::new(1.0, pole_radius, pole_radius);
    let pole_scale_z = Vec3::new(pole_radius, pole_radius, 1.0);

    commands
        .spawn_bundle(PbrBundle {
            mesh: unit_cube.clone(),
            material: pole_material.clone(),
            transform: Transform::from_matrix(
                Mat4::from_scale_rotation_translation(
                    pole_scale_x,
                    Quat::IDENTITY,
                    Vec3::new(0.0, pole_height, -0.5),
                ),
            ),
            ..Default::default()
        })
        .insert(Pole {
            goal_location: GoalLocation::Top,
        })
        .insert(Collider::Pole);

    commands
        .spawn_bundle(PbrBundle {
            mesh: unit_cube.clone(),
            material: pole_material.clone(),
            transform: Transform::from_matrix(
                Mat4::from_scale_rotation_translation(
                    pole_scale_z,
                    Quat::IDENTITY,
                    Vec3::new(0.5, pole_height, 0.0),
                ),
            ),
            ..Default::default()
        })
        .insert(Pole {
            goal_location: GoalLocation::Right,
        })
        .insert(Collider::Pole);

    commands
        .spawn_bundle(PbrBundle {
            mesh: unit_cube.clone(),
            material: pole_material.clone(),
            transform: Transform::from_matrix(
                Mat4::from_scale_rotation_translation(
                    pole_scale_x,
                    Quat::IDENTITY,
                    Vec3::new(0.0, pole_height, 0.5),
                ),
            ),
            ..Default::default()
        })
        .insert(Pole {
            goal_location: GoalLocation::Bottom,
        })
        .insert(Collider::Pole);

    commands
        .spawn_bundle(PbrBundle {
            mesh: unit_cube.clone(),
            material: pole_material.clone(),
            transform: Transform::from_matrix(
                Mat4::from_scale_rotation_translation(
                    pole_scale_z,
                    Quat::IDENTITY,
                    Vec3::new(-0.5, pole_height, 0.0),
                ),
            ),
            ..Default::default()
        })
        .insert(Pole {
            goal_location: GoalLocation::Left,
        })
        .insert(Collider::Pole);

    // Scores
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                position: Rect {
                    top: Val::Px(5.0),
                    right: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "",
                TextStyle {
                    font: font.clone(),
                    font_size: 50.0,
                    color: Color::RED,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    vertical: VerticalAlign::Center,
                },
            ),
            ..Default::default()
        })
        .insert(Score {
            goal_location: GoalLocation::Right,
        });

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                position: Rect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "",
                TextStyle {
                    font: font.clone(),
                    font_size: 50.0,
                    color: Color::RED,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    vertical: VerticalAlign::Center,
                },
            ),
            ..Default::default()
        })
        .insert(Score {
            goal_location: GoalLocation::Bottom,
        });

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                position: Rect {
                    bottom: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "",
                TextStyle {
                    font: font.clone(),
                    font_size: 50.0,
                    color: Color::RED,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    vertical: VerticalAlign::Center,
                },
            ),
            ..Default::default()
        })
        .insert(Score {
            goal_location: GoalLocation::Left,
        });

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "",
                TextStyle {
                    font: font.clone(),
                    font_size: 50.0,
                    color: Color::RED,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    vertical: VerticalAlign::Center,
                },
            ),
            ..Default::default()
        })
        .insert(Score {
            goal_location: GoalLocation::Top,
        });
}

fn setup_playable_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Crabs
    let unit_cube = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let crab_scale = Vec3::splat(0.1);
    let crab_height = 0.05;

    // Orange Crab
    commands
        .spawn_bundle(PbrBundle {
            mesh: unit_cube.clone(),
            material: materials.add(Color::ORANGE.into()),
            transform: Transform::from_matrix(
                Mat4::from_scale_rotation_translation(
                    crab_scale,
                    Quat::IDENTITY,
                    Vec3::new(0.0, crab_height, -0.5),
                ),
            ),
            ..Default::default()
        })
        .insert(Crab {
            goal_location: GoalLocation::Top,
            direction: CrabMovementDirection::Stop,
        })
        .insert(Movable {})
        .insert(Collider::Crab);

    // Blue Crab
    commands
        .spawn_bundle(PbrBundle {
            mesh: unit_cube.clone(),
            material: materials.add(Color::BLUE.into()),
            transform: Transform::from_matrix(
                Mat4::from_scale_rotation_translation(
                    crab_scale,
                    Quat::IDENTITY,
                    Vec3::new(0.5, crab_height, 0.0),
                ),
            ),
            ..Default::default()
        })
        .insert(Crab {
            goal_location: GoalLocation::Right,
            direction: CrabMovementDirection::Stop,
        })
        .insert(Movable {})
        .insert(Collider::Crab);

    // Red Crab
    commands
        .spawn_bundle(PbrBundle {
            mesh: unit_cube.clone(),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_matrix(
                Mat4::from_scale_rotation_translation(
                    crab_scale,
                    Quat::IDENTITY,
                    Vec3::new(0.0, crab_height, 0.5),
                ),
            ),
            ..Default::default()
        })
        .insert(Crab {
            goal_location: GoalLocation::Bottom,
            direction: CrabMovementDirection::Stop,
        })
        .insert(Movable {})
        .insert(Collider::Crab);

    // Purple Crab
    commands
        .spawn_bundle(PbrBundle {
            mesh: unit_cube.clone(),
            material: materials.add(Color::PURPLE.into()),
            transform: Transform::from_matrix(
                Mat4::from_scale_rotation_translation(
                    crab_scale,
                    Quat::IDENTITY,
                    Vec3::new(-0.5, crab_height, 0.0),
                ),
            ),
            ..Default::default()
        })
        .insert(Crab {
            goal_location: GoalLocation::Left,
            direction: CrabMovementDirection::Stop,
        })
        .insert(Movable {})
        .insert(Collider::Crab);

    // Balls
    let unit_sphere = meshes.add(Mesh::from(shape::Icosphere {
        radius: 1.5,
        subdivisions: 2,
    }));
    let ball_scale = Vec3::splat(0.05);
    let ball_height = 0.1;
    let ball_color = Color::rgb(1.0, 1.0, 1.0);

    commands
        .spawn_bundle(PbrBundle {
            mesh: unit_sphere.clone(),
            material: materials.add(ball_color.into()),
            transform: Transform::from_matrix(
                Mat4::from_scale_rotation_translation(
                    ball_scale,
                    Quat::IDENTITY,
                    Vec3::new(0.0, ball_height, 0.0),
                ),
            ),
            ..Default::default()
        })
        .insert(Ball {})
        .insert(Movable {})
        .insert(Collider::Ball);

    commands
        .spawn_bundle(PbrBundle {
            mesh: unit_sphere.clone(),
            material: materials.add(ball_color.into()),
            transform: Transform::from_matrix(
                Mat4::from_scale_rotation_translation(
                    ball_scale,
                    Quat::IDENTITY,
                    Vec3::new(0.0, ball_height, 0.0),
                ),
            ),
            ..Default::default()
        })
        .insert(Ball {})
        .insert(Movable {})
        .insert(Collider::Ball);
}

fn swaying_camera_system(
    config: Res<GameConfig>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut SwayingCamera)>,
) {
    // Slowly sway the camera back and forth
    let (mut transform, mut swaying_camera) = query.single_mut();
    let x = swaying_camera.angle.sin() * 0.5;

    *transform =
        Transform::from_xyz(x, 2.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y);

    swaying_camera.angle += config.swaying_camera_speed * time.delta_seconds();
    swaying_camera.angle %= std::f32::consts::TAU;
}

fn animated_water_system(
    config: Res<GameConfig>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut AnimatedWater)>,
) {
    // Translate the plane on the Z-axis, since we currently can't animate the
    // texture coordinates.
    let (mut transform, mut animated_water) = query.single_mut();

    *transform = Transform::from_xyz(0.0, -0.01, animated_water.scroll);

    animated_water.scroll += config.animated_water_speed * time.delta_seconds();
    animated_water.scroll %= 1.0;
}

fn crab_score_system(game: Res<Game>, mut query: Query<(&mut Text, &Score)>) {
    for (mut text, score) in query.iter_mut() {
        let score_value = game.scores[&score.goal_location];
        text.sections[0].value = score_value.to_string();
    }
}

fn crab_movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Crab)>,
) {
    for (mut transform, crab) in query.iter_mut() {
        let left_direction = match crab.goal_location {
            GoalLocation::Top => Vec3::new(1.0, 0.0, 0.0),
            GoalLocation::Right => Vec3::new(0.0, 0.0, 1.0),
            GoalLocation::Bottom => Vec3::new(-1.0, 0.0, 0.0),
            GoalLocation::Left => Vec3::new(0.0, 0.0, -1.0),
        };
        let sign = match crab.direction {
            CrabMovementDirection::Left => 1.0,
            CrabMovementDirection::Right => -1.0,
            _ => 0.0,
        };

        transform.translation += sign * left_direction * time.delta_seconds();
    }
}

fn player_crab_control_system(
    game: Res<Game>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Transform, &mut Crab)>,
) {
    for (_, mut crab) in query.iter_mut() {
        if crab.goal_location == game.player_goal_location {
            if keyboard_input.pressed(KeyCode::Left) {
                crab.direction = CrabMovementDirection::Left;
            } else if keyboard_input.pressed(KeyCode::Right) {
                crab.direction = CrabMovementDirection::Right;
            } else {
                crab.direction = CrabMovementDirection::Stop;
            }
        }
    }
}

fn ai_crab_control_system(
    game: Res<Game>,
    mut query: Query<(&Transform, &mut Crab)>,
) {
    // for (_, mut crab) in query.iter_mut() {
    //     if crab.goal_location != game.player_goal_location {
    //         if keyboard_input.pressed(KeyCode::Left) {
    //             crab.direction = CrabMovementDirection::Left;
    //         } else if keyboard_input.pressed(KeyCode::Right) {
    //             crab.direction = CrabMovementDirection::Right;
    //         } else {
    //             crab.direction = CrabMovementDirection::Idle;
    //         }
    //     }
    // }

    // TODO: Start with test code that randomly picks spots where it will think
    // the ball is approaching and then have them move towards that spot.
}

fn ball_collision_system() {}

fn ball_movement_system() {}
