use bevy::{ecs::prelude::*, prelude::*};
use lazy_static::*;
use serde::Deserialize;
use std::collections::HashMap;

pub mod animated_water;
pub use animated_water::*;

pub mod ball;
pub use ball::*;

pub mod barrier;
pub use barrier::*;

pub mod enemy;
pub use enemy::*;

pub mod fade;
pub use fade::*;

pub mod gameover_message;
pub use gameover_message::*;

pub mod goal;
pub use goal::*;

pub mod movement;
pub use movement::*;

pub mod paddle;
pub use paddle::*;

pub mod player;
pub use player::*;

pub mod score;
pub use score::*;

pub mod swaying_camera;
pub use swaying_camera::*;

pub mod wall;
pub use wall::*;

pub const ARENA_WIDTH: f32 = 1.0;
pub const ARENA_HALF_WIDTH: f32 = 0.5 * ARENA_WIDTH;
pub const BARRIER_DIAMETER: f32 = 0.20;
pub const BARRIER_RADIUS: f32 = 0.5 * BARRIER_DIAMETER;
pub const BALL_HEIGHT: f32 = 0.05;
pub const BALL_DIAMETER: f32 = 0.1;
pub const BALL_RADIUS: f32 = 0.5 * BALL_DIAMETER;
pub const PADDLE_WIDTH: f32 = 0.2;
pub const PADDLE_HALF_WIDTH: f32 = 0.5 * PADDLE_WIDTH;
pub const PADDLE_DEPTH: f32 = 0.1;
pub const PADDLE_HALF_DEPTH: f32 = 0.5 * PADDLE_DEPTH;
pub const PADDLE_MAX_POSITION_X: f32 =
    ARENA_HALF_WIDTH - BARRIER_RADIUS - PADDLE_HALF_WIDTH;
pub const WALL_DIAMETER: f32 = 0.05;
pub const WALL_RADIUS: f32 = 0.5 * WALL_DIAMETER;

lazy_static! {
    pub static ref ARENA_CENTER_POINT: Vec3 = Vec3::ZERO;
    pub static ref BALL_CENTER_POINT: Vec3 =
        Vec3::new(ARENA_CENTER_POINT.x, BALL_HEIGHT, ARENA_CENTER_POINT.z);
    pub static ref PADDLE_SCALE: Vec3 =
        Vec3::new(PADDLE_WIDTH, PADDLE_DEPTH, PADDLE_DEPTH);
    pub static ref PADDLE_START_POSITION: Vec3 = Vec3::new(0.0, 0.05, 0.0);
    pub static ref WALL_SCALE: Vec3 =
        Vec3::new(ARENA_WIDTH, WALL_DIAMETER, WALL_DIAMETER);
}

/// The current state of the game.
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Playing,
    GameOver,
}

/// All global information for this game.
#[derive(Default)]
pub struct Game {
    pub scores: HashMap<Goal, u32>,
    pub over: Option<GameOver>,
}

/// Game settings read from a `*.ron` config file.
#[derive(Debug, Deserialize)]
pub struct GameConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub clear_color: Color,
    pub swaying_camera_speed: f32,
    pub animated_water_speed: f32,
    pub paddle_max_speed: f32,
    pub paddle_seconds_to_max_speed: f32,
    pub ball_starting_speed: f32,
    pub ball_max_speed: f32,
    pub ball_seconds_to_max_speed: f32,
    pub fade_speed: f32,
    pub starting_score: u32,
}

/// Represents whether the player won or lost the last game.
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum GameOver {
    Won,
    Lost,
}

impl Default for GameOver {
    fn default() -> Self { Self::Won }
}

/// Handles setting up all the entities that will be needed for every screen of
/// this game.
pub fn setup(
    mut game: ResMut<Game>,
    config: Res<GameConfig>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let unit_plane = meshes.add(Mesh::from(shape::Plane { size: 1.0 }));

    // Cameras
    commands
        .spawn_bundle(PerspectiveCameraBundle::default())
        .insert(SwayingCamera::default());

    commands.spawn_bundle(UiCameraBundle::default());

    // Light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // Ocean
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
            material: materials.add(Color::hex("257AFF").unwrap().into()),
            transform: Transform::from_xyz(0.0, -0.01, 0.0),
            ..Default::default()
        })
        .insert(AnimatedWater::default());

    // Beach
    commands.spawn_bundle(PbrBundle {
        mesh: unit_plane.clone(),
        material: materials.add(Color::hex("C4BD99").unwrap().into()),
        transform: Transform::from_matrix(
            Mat4::from_scale_rotation_translation(
                Vec3::splat(ARENA_WIDTH),
                Quat::IDENTITY,
                *ARENA_CENTER_POINT,
            ),
        ),
        ..Default::default()
    });

    // Balls
    let unit_sphere = meshes.add(Mesh::from(shape::Icosphere {
        radius: 0.5,
        subdivisions: 2,
    }));
    let ball_material = materials.add(Color::WHITE.into());

    for _ in 0..2 {
        commands
            .spawn_bundle(PbrBundle {
                mesh: unit_sphere.clone(),
                material: ball_material.clone(),
                transform: Transform::from_matrix(
                    Mat4::from_scale_rotation_translation(
                        Vec3::splat(BALL_DIAMETER),
                        Quat::IDENTITY,
                        *BALL_CENTER_POINT,
                    ),
                ),
                ..Default::default()
            })
            .insert_bundle((Ball, Fade::Out(1.0)));
    }

    // Goals
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    let unit_cube = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let wall_material = materials.add(Color::hex("00A400").unwrap().into());
    let barrier_material = materials.add(Color::hex("750000").unwrap().into());
    let goal_configs = [
        (
            Color::RED,
            Goal::Bottom,
            Rect {
                bottom: Val::Px(5.0),
                right: Val::Px(400.0),
                ..Default::default()
            },
        ),
        (
            Color::BLUE,
            Goal::Right,
            Rect {
                top: Val::Px(400.0),
                right: Val::Px(5.0),
                ..Default::default()
            },
        ),
        (
            Color::ORANGE,
            Goal::Top,
            Rect {
                top: Val::Px(5.0),
                left: Val::Px(400.0),
                ..Default::default()
            },
        ),
        (
            Color::PURPLE,
            Goal::Left,
            Rect {
                bottom: Val::Px(400.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
        ),
    ];

    for (i, (color, goal, rect)) in goal_configs.iter().enumerate() {
        // Goal
        commands
            .spawn_bundle(PbrBundle {
                transform: Transform::from_rotation(Quat::from_axis_angle(
                    Vec3::Y,
                    std::f32::consts::TAU
                        * (i as f32 / goal_configs.len() as f32),
                ))
                .mul_transform(Transform::from_xyz(0.0, 0.0, ARENA_HALF_WIDTH)),
                ..Default::default()
            })
            .with_children(|parent| {
                // Paddle
                // NOTE: Treat it as the center of the goal
                parent
                    .spawn_bundle(PbrBundle {
                        mesh: unit_cube.clone(),
                        material: materials.add(color.clone().into()),
                        transform: Transform::from_matrix(
                            Mat4::from_scale_rotation_translation(
                                *PADDLE_SCALE,
                                Quat::IDENTITY,
                                *PADDLE_START_POSITION,
                            ),
                        ),
                        ..Default::default()
                    })
                    .insert_bundle((Paddle, Fade::Out(1.0), goal.clone()));

                // Wall
                parent
                    .spawn_bundle(PbrBundle {
                        mesh: unit_cube.clone(),
                        material: wall_material.clone(),
                        transform: Transform::from_matrix(
                            Mat4::from_scale_rotation_translation(
                                *WALL_SCALE,
                                Quat::IDENTITY,
                                Vec3::new(0.0, 0.1, 0.0),
                            ),
                        ),
                        ..Default::default()
                    })
                    .insert_bundle((Wall, Active, goal.clone()));

                // Barrier
                parent
                    .spawn_bundle(PbrBundle {
                        mesh: unit_cube.clone(),
                        material: barrier_material.clone(),
                        transform: Transform::from_matrix(
                            Mat4::from_scale_rotation_translation(
                                Vec3::splat(BARRIER_DIAMETER),
                                Quat::IDENTITY,
                                Vec3::new(ARENA_HALF_WIDTH, 0.1, 0.0),
                            ),
                        ),
                        ..Default::default()
                    })
                    .insert(Barrier);
            });

        // Score
        commands
            .spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    position: *rect,
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
            .insert_bundle((Score, goal.clone()));

        game.scores.insert(goal.clone(), 0);
    }

    // Gameover message
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            material: color_materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(
                            Val::Percent(100.0),
                            Val::Percent(100.0),
                        ),
                        position_type: PositionType::Absolute,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    material: color_materials.add(Color::NONE.into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                margin: Rect::all(Val::Px(5.0)),
                                ..Default::default()
                            },
                            text: Text::with_section(
                                "",
                                TextStyle {
                                    font: asset_server
                                        .load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 30.0,
                                    color: Color::RED,
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(GameoverMessage);
                });
        });
}

/// When entering the gameover screen shows the corresponding UI and says
/// whether the player won/lost.
pub fn show_gameover_ui(
    game: Res<Game>,
    mut query: Query<&mut Text, With<GameoverMessage>>,
) {
    let mut text = query.single_mut();

    text.sections[0].value = String::new();

    if let Some(game_over) = game.over {
        if game_over == GameOver::Won {
            text.sections[0].value.push_str("You win!\n");
        } else {
            text.sections[0].value.push_str("You lose.\n");
        }

        text.sections[0].value.push_str("\n");
    }

    text.sections[0].value.push_str(
        "Press ENTER for a new game\nPress ESC to quit\n(Use left and right \
         to move)",
    );
}

/// Handles keyboard inputs and launching a new game when on the gameover
/// screen.
pub fn gameover_keyboard_system(
    mut state: ResMut<State<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        state.set(GameState::Playing).unwrap();
    }
}

/// Hides the gameover UI at the start of a new game.
pub fn hide_gameover_ui(mut query: Query<&mut Text, With<GameoverMessage>>) {
    let mut text = query.single_mut();
    text.sections[0].value = "".to_owned();
}

/// Resets the state of all the goals and their scores when starting a new game.
pub fn reset_game_entities(
    mut commands: Commands,
    config: Res<GameConfig>,
    mut game: ResMut<Game>,
    query: Query<(Entity, &Goal), With<Paddle>>,
    mut paddles_query: Query<
        (Entity, &mut Transform),
        (With<Paddle>, Without<Active>),
    >,
    walls_query: Query<Entity, (With<Wall>, With<Active>)>,
) {
    // Assign players
    for (entity, goal) in query.iter() {
        // TODO: Build this out to support more crazy configurations that can be
        // set at runtime
        if *goal == Goal::Bottom {
            commands.entity(entity).insert(Player);
        } else {
            commands.entity(entity).insert(Enemy);
        }
    }

    // Reset paddles
    for (entity, mut transform) in paddles_query.iter_mut() {
        transform.translation = *PADDLE_START_POSITION;
        commands.entity(entity).insert(Fade::In(0.4));
    }

    // Reset walls
    for entity in walls_query.iter() {
        commands.entity(entity).insert(Fade::Out(0.3));
    }

    // Reset scores
    for (goal, score) in game.scores.iter_mut() {
        *score = config.starting_score;

        // HACK: Makes debugging simpler for now
        if *goal == Goal::Bottom {
            *score = 99;
        }
    }
}

/// When a goal is eliminated it checks if the current scores of all the goals
/// have triggered a gameover.
pub fn gameover_check_system(
    mut game: ResMut<Game>,
    mut state: ResMut<State<GameState>>,
    mut goal_eliminated_reader: EventReader<GoalEliminated>,
    enemies_query: Query<&Goal, (With<Paddle>, With<Enemy>)>,
    players_query: Query<&Goal, (With<Paddle>, With<Player>)>,
) {
    for GoalEliminated(_) in goal_eliminated_reader.iter() {
        // Player wins if all Enemy paddles have a score of zero
        let has_player_won =
            enemies_query.iter().all(|goal| game.scores[&goal] == 0);

        // Player loses if all Player paddles have a score of zero
        let has_player_lost =
            players_query.iter().all(|goal| game.scores[&goal] == 0);

        // Declare a winner and trigger gameover
        if has_player_won || has_player_lost {
            game.over = if has_player_won {
                Some(GameOver::Won)
            } else {
                Some(GameOver::Lost)
            };

            state.set(GameState::GameOver).unwrap();
        }
    }
}

/// Fades out and deactivates any `Ball` entities that are still in play at the
/// beginning of a gameover.
pub fn fade_out_balls(
    mut commands: Commands,
    query: Query<(Entity, Option<&Fade>, Option<&Active>), With<Ball>>,
) {
    for (entity, fade, active) in query.iter() {
        match fade {
            Some(Fade::In(weight)) => {
                commands.entity(entity).insert(Fade::Out(1.0 - weight));
            },
            None if active.is_some() => {
                commands.entity(entity).insert(Fade::Out(0.0));
            },
            _ => {},
        }
    }
}

// TODO: Move scores closer to the paddles.

// TODO: Add win/lose messages.

// TODO: Widen the size of field so that balls have to go fully past paddles to
// be considered out of bounds, rather than now where they can intersect halfway
// through the paddle and be considered out of bounds.

// TODO: Add event logging.

// TODO: Need to fix rare issue where restarting the game too quickly can make a
// wall disappear, but it still deflects balls as though it was there. May be
// related to it being the last goal before gameover?

// TODO: Need a fix for the rare occasion when a ball just bounces infinitely
// between two walls in a straight line?

// TODO: Offer a "Traditional" mode with two paddles (1xPlayer, 1xEnemy)
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
