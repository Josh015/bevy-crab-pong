mod files;

use std::collections::HashMap;

use bevy::{
    ecs::prelude::*,
    prelude::*,
    render::camera::{Camera, PerspectiveProjection},
};
use serde::Deserialize;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum CrabColor {
    Orange,
    Blue,
    Red,
    Purple,
}

enum CrabMovementDirection {
    Idle,
    Left,
    Right,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Playing,
    GameOver,
}

// #[derive(Component)]
struct Score {
    crab_color: CrabColor,
}

// #[derive(Component)]
struct Water {
    scroll: f32,
}

// #[derive(Component)]
struct Crab {
    color: CrabColor,
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
    crab_color: CrabColor,
    // is_active: bool,
}

// #[derive(Component)]
struct Movable {}

// #[derive(Component)]
struct Collider {}

#[derive(Debug, Deserialize)]
struct GameConfig {
    title: String,
    width: u32,
    height: u32,
    camera_sway_speed: f32,
    /* startingScore: u8, //20,
     * crabSpeed: f32,    // 2.2,
     * ballSpeed: f32,    // ?? */
}

struct Game {
    scores: HashMap<CrabColor, u32>,
    camera_angle: f32,
    player_crab_color: CrabColor,
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
                (CrabColor::Orange, 20),
                (CrabColor::Blue, 20),
                (CrabColor::Red, 20),
                (CrabColor::Purple, 20),
            ]),
            player_crab_color: CrabColor::Red,
            camera_angle: 0.0,
        })
        .add_startup_system(setup_level)
        .add_startup_system(setup_playable_entities)
        .add_system(animate_water)
        .add_system(sway_camera)
        .add_system(display_scores)
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

    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 2.5, 5.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    let unit_plane = meshes.add(Mesh::from(shape::Plane { size: 1.0 }));

    // Ocean
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
            material: materials.add(Color::hex("257AFF").unwrap().into()),
            transform: Transform::from_xyz(0.0, -0.01, 0.0),
            ..Default::default()
        })
        .insert(Water { scroll: 0.0 });

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
        .insert(Collider {});

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
        .insert(Collider {});

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
        .insert(Collider {});

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
        .insert(Collider {});

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
            crab_color: CrabColor::Orange,
        })
        .insert(Collider {});

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
            crab_color: CrabColor::Blue,
        })
        .insert(Collider {});

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
            crab_color: CrabColor::Red,
        })
        .insert(Collider {});

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
            crab_color: CrabColor::Purple,
        })
        .insert(Collider {});

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
            crab_color: CrabColor::Blue,
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
            crab_color: CrabColor::Red,
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
            crab_color: CrabColor::Purple,
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
            crab_color: CrabColor::Orange,
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
            color: CrabColor::Orange,
            direction: CrabMovementDirection::Idle,
        })
        .insert(Movable {})
        .insert(Collider {});

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
            color: CrabColor::Blue,
            direction: CrabMovementDirection::Idle,
        })
        .insert(Movable {})
        .insert(Collider {});

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
            color: CrabColor::Red,
            direction: CrabMovementDirection::Idle,
        })
        .insert(Movable {})
        .insert(Collider {});

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
            color: CrabColor::Purple,
            direction: CrabMovementDirection::Idle,
        })
        .insert(Movable {})
        .insert(Collider {});

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
        .insert(Collider {});

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
        .insert(Collider {});
}

fn sway_camera(
    config: Res<GameConfig>,
    mut game: ResMut<Game>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Camera, &PerspectiveProjection)>,
) {
    // Slowly sway the camera back and forth
    let (mut transform, _, _) = query.single_mut();
    let x = game.camera_angle.sin() * 0.5;

    game.camera_angle += config.camera_sway_speed * time.delta_seconds();
    game.camera_angle %= std::f32::consts::TAU;

    *transform =
        Transform::from_xyz(x, 2.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y);
}

fn animate_water(
    config: Res<GameConfig>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Water)>,
) {
    // TODO: Animate water texture coordinates in the +Y direction.
    // TODO: .single_mut()
}

// TODO: Should this be event-based, since the scores only updates on goals or
// game over?
fn display_scores(game: Res<Game>, mut query: Query<(&mut Text, &Score)>) {
    for (mut text, score) in query.iter_mut() {
        let score_value = game.scores[&score.crab_color];
        text.sections[0].value = score_value.to_string();
    }
}

// --Systems--
// One update function for all crabs? Skip for player_crab_color.
// * load_initial_scene()
// * load_game_over_scene()
// * load_new_game_scene() Delay before spawning balls?
// * update_scores() Crab whose score hits zero needs to have direction set to
//   Idle and speed set to zero immediately!
// * player_input()
// * enemy_ai() May need to work on a fixed timestep.
// * move_balls() // Handles ball resets as well?

// -- General --
// Instead of mirroring, have reflections be child entities and just move one
// entity?      What about if we want to add animation in the future?
