mod components;
mod config;
mod files;
mod state;
mod util;

pub mod prelude {
    pub use crate::{components::*, config::*, state::*, util::*};
    pub use bevy::{math::*, prelude::*};
    pub use rand::prelude::*;
}

use bevy::{
    app::AppExit,
    window::{PresentMode, WindowResolution},
};

use crate::prelude::*;

fn main() {
    let config: GameConfig =
        files::load_config_from_file("assets/config/game.ron");

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: config.title.clone(),
                resolution: WindowResolution::new(
                    config.width as f32,
                    config.height as f32,
                ),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Msaa::default())
        .insert_resource(ClearColor(config.clear_color))
        .insert_resource(config)
        .add_plugin(StatePlugin)
        .add_plugin(ComponentsPlugin)
        .add_system(input)
        .add_startup_system(setup)
        .run();
}

/// Handles all user input regardless of the current game state.
fn input(
    keyboard_input: Res<Input<KeyCode>>,
    game_screen: Res<State<GameScreen>>,
    mut next_game_screen: ResMut<NextState<GameScreen>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
    } else if game_screen.0 != GameScreen::StartMenu
        && keyboard_input.just_pressed(KeyCode::Back)
    {
        next_game_screen.set(GameScreen::StartMenu);
        info!("Start Menu");
    } else if game_screen.0 == GameScreen::StartMenu {
        if keyboard_input.just_pressed(KeyCode::Return) {
            next_game_screen.set(GameScreen::Playing);
            info!("New Game");
        }
    } else if game_screen.0 == GameScreen::Playing {
        if keyboard_input.just_pressed(KeyCode::Space) {
            next_game_screen.set(GameScreen::Paused);
            info!("Paused");
        }
    } else if game_screen.0 == GameScreen::Paused
        && keyboard_input.just_pressed(KeyCode::Space)
    {
        next_game_screen.set(GameScreen::Playing);
        info!("Unpaused");
    }
}

/// Handles setting up the static arena entities.
fn setup(
    mut run_state: ResMut<RunState>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spawn_wall_events: EventWriter<SpawnWallEvent>,
) {
    // Cameras
    commands.spawn((SwayingCamera, Camera3dBundle::default()));

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
        AnimatedWater::default(),
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
    let goal_configs = [
        (
            Side::Bottom,
            UiRect {
                bottom: Val::Px(5.0),
                right: Val::Px(400.0),
                ..default()
            },
        ),
        (
            Side::Right,
            UiRect {
                top: Val::Px(400.0),
                right: Val::Px(5.0),
                ..default()
            },
        ),
        (
            Side::Top,
            UiRect {
                top: Val::Px(5.0),
                left: Val::Px(400.0),
                ..default()
            },
        ),
        (
            Side::Left,
            UiRect {
                bottom: Val::Px(400.0),
                left: Val::Px(5.0),
                ..default()
            },
        ),
    ];

    for (i, (side, rect)) in goal_configs.iter().enumerate() {
        // Walls
        spawn_wall_events.send(SpawnWallEvent {
            side: *side,
            is_instant: true,
        });

        // Goals
        commands
            .spawn((
                *side,
                Goal,
                PbrBundle {
                    transform: Transform::from_rotation(Quat::from_axis_angle(
                        Vec3::Y,
                        std::f32::consts::TAU
                            * (i as f32 / goal_configs.len() as f32),
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
            });

        // Score
        commands.spawn((
            HitPointsUi,
            *side,
            TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    position: *rect,
                    ..default()
                },
                text: Text::from_section(
                    "",
                    TextStyle {
                        font: run_state.font_handle.clone(),
                        font_size: 50.0,
                        color: Color::RED,
                    },
                ),
                ..default()
            },
        ));

        run_state.goals_hit_points.insert(*side, 0);
    }
}

// TODO: Need a fix for the rare occasion when a ball just bounces infinitely
// between two walls in a straight line? Maybe make all bounces slightly adjust
// ball angle rather than pure reflection?

// TODO: Offer a "Traditional" mode with two paddles (1xPlayer, 1xAi)
// opposite each other and the other two walled off. Also just one ball?

// TODO: Debug option to make all paddles driven by AI? Will need to revise
// player system to handle no players.

// TODO: Debug option to directly control single ball's exact position with
// keyboard and see how paddles respond. Can go in goals, triggering a score and
// ball return?

// TODO: Debug option to add small cubes at the projected points on goals with
// debug lines to the nearest ball. Also add a line from the paddle to a flat
// but wide cube (to allow both to be visible if they overlap) that matches the
// paddle's hit box dimensions and is positioned where the paddle predicts it
// will stop. One of each per goal so we can spawn them in advance.
