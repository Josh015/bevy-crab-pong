use bevy::{ecs::prelude::*, prelude::*};
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

pub mod goal;
pub use goal::*;

pub mod paddle;
pub use paddle::*;

pub mod player;
pub use player::*;

pub mod score;
pub use score::*;

pub mod swaying_camera;
pub use swaying_camera::*;

pub mod velocity;
pub use velocity::*;

pub mod wall;
pub use wall::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Playing,
    GameOver,
}

#[derive(Default)]
pub struct Game {
    pub scores: HashMap<Goal, u32>,
    pub over: Option<GameOver>,
}

#[derive(Debug, Deserialize)]
pub struct GameConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub clear_color: Color,
    pub swaying_camera_speed: f32,
    pub animated_water_speed: f32,
    pub beach_center_point: (f32, f32, f32),
    pub beach_width: f32,
    pub paddle_max_speed: f32,
    pub paddle_seconds_to_max_speed: f32,
    pub paddle_scale: (f32, f32, f32),
    pub paddle_start_position: (f32, f32, f32),
    pub ball_size: f32,
    pub ball_height: f32,
    pub ball_speed: f32,
    pub barrier_width: f32,
    pub fade_speed: f32,
    pub wall_radius: f32,
    pub starting_score: u32,
}

impl GameConfig {
    pub fn paddle_acceleration(&self) -> f32 {
        self.paddle_max_speed / self.paddle_seconds_to_max_speed
    }

    pub fn ball_center_point(&self) -> Vec3 {
        let mut ball_center_point: Vec3 = self.beach_center_point.into();
        ball_center_point.y = self.ball_height;
        ball_center_point
    }

    pub fn ball_radius(&self) -> f32 { 0.5 * self.ball_size }

    pub fn wall_scale(&self) -> Vec3 {
        Vec3::new(self.beach_width, self.wall_radius, self.wall_radius)
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum GameOver {
    Won,
    Lost,
}

impl Default for GameOver {
    fn default() -> Self { Self::Won }
}

pub fn setup(
    mut game: ResMut<Game>,
    config: Res<GameConfig>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
                Vec3::splat(config.beach_width),
                Quat::IDENTITY,
                config.beach_center_point.into(),
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
                        Vec3::splat(config.ball_size),
                        Quat::IDENTITY,
                        config.ball_center_point(),
                    ),
                ),
                ..Default::default()
            })
            .insert_bundle((Ball, Fade::Out(1.0)));
    }

    // Goals
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let unit_cube = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let wall_material = materials.add(Color::hex("00A400").unwrap().into());
    let barrier_material = materials.add(Color::hex("750000").unwrap().into());
    let goal_configs = [
        (
            Color::RED,
            Goal::Bottom,
            Rect {
                bottom: Val::Px(5.0),
                right: Val::Px(5.0),
                ..Default::default()
            },
        ),
        (
            Color::BLUE,
            Goal::Right,
            Rect {
                top: Val::Px(5.0),
                right: Val::Px(5.0),
                ..Default::default()
            },
        ),
        (
            Color::ORANGE,
            Goal::Top,
            Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
        ),
        (
            Color::PURPLE,
            Goal::Left,
            Rect {
                bottom: Val::Px(5.0),
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
                .mul_transform(Transform::from_xyz(
                    0.0,
                    0.0,
                    0.5 * config.beach_width,
                )),
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
                                config.paddle_scale.into(),
                                Quat::IDENTITY,
                                config.paddle_start_position.into(),
                            ),
                        ),
                        ..Default::default()
                    })
                    .insert_bundle((
                        Paddle::Stopped,
                        Fade::Out(1.0),
                        Velocity {
                            speed: 0.0,
                            direction: Vec3::X,
                        },
                        goal.clone(),
                    ));

                // Wall
                parent
                    .spawn_bundle(PbrBundle {
                        mesh: unit_cube.clone(),
                        material: wall_material.clone(),
                        transform: Transform::from_matrix(
                            Mat4::from_scale_rotation_translation(
                                config.wall_scale(),
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
                                Vec3::splat(config.barrier_width),
                                Quat::IDENTITY,
                                Vec3::new(0.5 * config.beach_width, 0.1, 0.0),
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
}

pub fn show_gameover_ui(game: Res<Game>) {
    if let Some(game_over) = game.over {
        if game_over == GameOver::Won {
            // TODO: Add win text
        } else {
            // TODO: Add loss text
        }
    }

    // Show instructions for new game
    // TODO: new game text visible
}

pub fn gameover_keyboard_system(
    mut state: ResMut<State<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        state.set(GameState::Playing).unwrap();
    }
}

pub fn hide_gameover_ui() {
    // TODO: Hide message text
}

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
        commands.entity(entity).insert(Fade::In(0.4));
        transform.translation = config.paddle_start_position.into();
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

pub fn gameover_check_system(
    mut game: ResMut<Game>,
    mut state: ResMut<State<GameState>>,
    mut goal_eliminated_reader: EventReader<GoalEliminated>,
    players_query: Query<&Goal, (With<Paddle>, With<Player>)>,
    enemies_query: Query<&Goal, (With<Paddle>, With<Enemy>)>,
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

pub fn fade_out_balls(
    mut commands: Commands,
    query: Query<(Entity, Option<&Active>, Option<&Fade>), With<Ball>>,
) {
    for (entity, active, fade) in query.iter() {
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

// NOTE: Rules for which modules get which systems:
// 1.) The component of the outermost query (eg. With<Ball>, With<Paddle>, etc.)
// is where that system should live.
// 2.) Especially the case if the system ultimately modifies that entity that
// owns that component.
// 3.) If it has multiple top-level queries, then it probably belongs in the
// game module.

// TODO: Go back through and remove un-needed imports.

// TODO: Go back through and remove un-needed iter_mut() and mutability calls.

// TODO: Add event logging.

// TODO: Need to document the hell out of this code.

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
