mod ecs;
mod files;

use bevy::{
    ecs::prelude::*, prelude::*, render::camera::PerspectiveProjection,
};
use ecs::{
    animated_water::*, ball::*, barrier::*, crab::*, enemy::*, fade::*,
    goal::*, player::*, score::*, swaying_camera::*, velocity::*, wall::*, *,
};
use rand::prelude::*;
use serde::Deserialize;
use std::{
    collections::HashMap,
    ops::{Add, Sub},
};

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
        .init_resource::<Game>()
        .insert_resource(config)
        .add_startup_system(setup_scene)
        .add_startup_system(setup_balls)
        .add_startup_system(setup_goals)
        .add_system(score::display_scores_system)
        .add_system(swaying_camera_system)
        .add_system(animated_water::animation_system)
        .add_system(fade::start_fade_system)
        .add_system(fade::step_fade_system)
        .add_system(crab::step_fade_animation_system)
        .add_system(wall::start_fade_system)
        .add_system(wall::step_fade_animation_system)
        .add_system(ball::step_fade_animation_system)
        .add_system(goal_eliminated_animation_system)
        .add_state(GameState::GameOver)
        .add_event::<GoalEliminated>()
        .add_system_set(
            SystemSet::on_enter(GameState::GameOver)
                .with_system(gameover_show_ui),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::GameOver)
                .with_system(gameover_hide_ui),
        )
        .add_system_set(
            SystemSet::on_update(GameState::GameOver)
                .with_system(gameover_keyboard_system),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(assign_players)
                .with_system(reset_game_entities),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Playing).with_system(fade_out_balls),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(crab::movement_system)
                .with_system(player::crab_control_system)
                .with_system(enemy::crab_control_system)
                .with_system(velocity::movement_system)
                .with_system(reset_ball_position_system)
                .with_system(reset_ball_velocity_system)
                .with_system(collision_system)
                .with_system(goal_scored_system)
                .with_system(gameover_check_system),
        )
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

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
    title: String,
    width: u32,
    height: u32,
    swaying_camera_speed: f32,
    animated_water_speed: f32,
    beach_center_point: (f32, f32, f32),
    beach_width: f32,
    crab_max_speed: f32,
    crab_seconds_to_max_speed: f32,
    crab_scale: (f32, f32, f32),
    crab_start_position: (f32, f32, f32),
    ball_size: f32,
    ball_height: f32,
    ball_speed: f32,
    barrier_width: f32,
    fade_speed: f32,
    wall_radius: f32,
    starting_score: u32,
}

impl GameConfig {
    fn crab_acceleration(&self) -> f32 {
        self.crab_max_speed / self.crab_seconds_to_max_speed
    }

    fn ball_center_point(&self) -> Vec3 {
        let mut ball_center_point: Vec3 = self.beach_center_point.into();
        ball_center_point.y = self.ball_height;
        ball_center_point
    }

    fn ball_radius(&self) -> f32 { 0.5 * self.ball_size }

    fn wall_scale(&self) -> Vec3 {
        Vec3::new(self.beach_width, self.wall_radius, self.wall_radius)
    }
}

struct GoalEliminated(Goal);

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum GameOver {
    Won,
    Lost,
}

impl Default for GameOver {
    fn default() -> Self { Self::Won }
}

fn setup_scene(
    config: Res<GameConfig>,
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
}

fn setup_balls(
    config: Res<GameConfig>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
}

fn setup_goals(
    mut game: ResMut<Game>,
    config: Res<GameConfig>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
                // Crab
                // NOTE: Treat it as the center of the goal
                parent
                    .spawn_bundle(PbrBundle {
                        mesh: unit_cube.clone(),
                        material: materials.add(color.clone().into()),
                        transform: Transform::from_matrix(
                            Mat4::from_scale_rotation_translation(
                                config.crab_scale.into(),
                                Quat::IDENTITY,
                                config.crab_start_position.into(),
                            ),
                        ),
                        ..Default::default()
                    })
                    .insert_bundle((
                        Crab::default(),
                        Fade::Out(1.0),
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

fn goal_eliminated_animation_system(
    mut commands: Commands,
    mut goal_eliminated_reader: EventReader<GoalEliminated>,
    balls_query: Query<(Entity, &Goal), (With<Crab>, With<Active>)>,
    walls_query: Query<(Entity, &Goal), (With<Wall>, Without<Active>)>,
) {
    for GoalEliminated(eliminated_goal) in goal_eliminated_reader.iter() {
        for (entity, goal) in balls_query.iter() {
            if goal == eliminated_goal {
                commands.entity(entity).insert(Fade::Out(0.0));
                break;
            }
        }

        for (entity, goal) in walls_query.iter() {
            if goal == eliminated_goal {
                commands.entity(entity).insert(Fade::In(0.0));
                break;
            }
        }
    }
}

fn gameover_show_ui(game: Res<Game>) {
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

fn gameover_hide_ui() {
    // TODO: Hide message text
}

fn gameover_keyboard_system(
    mut state: ResMut<State<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        state.set(GameState::Playing).unwrap();
    }
}

fn assign_players(
    mut commands: Commands,
    query: Query<(Entity, &Goal), With<Crab>>,
) {
    for (entity, goal) in query.iter() {
        // TODO: Build this out to support more crazy configurations that can be
        // set at runtime
        if *goal == Goal::Bottom {
            commands.entity(entity).insert(Player);
        } else {
            commands.entity(entity).insert(Enemy);
        }
    }
}

fn reset_game_entities(
    mut commands: Commands,
    config: Res<GameConfig>,
    mut game: ResMut<Game>,
    mut crabs_query: Query<
        (Entity, &mut Transform),
        (With<Crab>, Without<Active>),
    >,
    walls_query: Query<Entity, (With<Wall>, With<Active>)>,
) {
    // Reset crabs
    for (entity, mut transform) in crabs_query.iter_mut() {
        commands.entity(entity).insert(Fade::In(0.4));
        transform.translation = config.crab_start_position.into();
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

fn fade_out_balls(
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

// TODO: This would be a good opportunity for a score event!

fn reset_ball_position_system(
    mut commands: Commands,
    config: Res<GameConfig>,
    mut query: Query<
        (Entity, &mut Transform),
        (With<Ball>, Without<Fade>, Without<Active>),
    >,
) {
    for (entity, mut transform) in query.iter_mut() {
        transform.translation = config.ball_center_point();
        commands
            .entity(entity)
            .remove::<Velocity>()
            .insert(Fade::In(0.0));
    }
}

fn reset_ball_velocity_system(
    mut commands: Commands,
    config: Res<GameConfig>,
    query: Query<Entity, (With<Ball>, Without<Velocity>, Added<Active>)>,
) {
    for entity in query.iter() {
        // TODO: Move this into a global resource? Also, make a reusable uniform
        // for random rotation?
        let mut rng = rand::thread_rng();

        // Give the ball a random direction vector
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let velocity =
            config.ball_speed * Vec3::new(angle.cos(), 0.0, angle.sin());

        commands.entity(entity).insert(Velocity(velocity));
    }
}

fn collision_system(
    mut commands: Commands,
    config: Res<GameConfig>,
    balls_query: Query<
        (Entity, &GlobalTransform, &Velocity),
        (With<Ball>, With<Active>),
    >,
    crabs_query: Query<&GlobalTransform, (With<Crab>, With<Active>)>,
    barriers_query: Query<&GlobalTransform, With<Barrier>>,
    walls_query: Query<(&GlobalTransform, &Goal), (With<Wall>, With<Active>)>,
) {
    for (entity, transform, velocity) in balls_query.iter() {
        let ball_radius = config.ball_radius();
        let barrier_radius = 0.5 * config.barrier_width;
        let half_width = 0.5 * config.beach_width;
        let d = velocity.0.normalize();
        let radius_position = transform.translation + d * ball_radius;

        // TODO: Order these so that highest precedence collision type is at the
        // bottom, since it can overwrite others!

        // Ball collisions
        for (entity2, transform2, velocity2) in balls_query.iter() {
            break;
        }

        // Crab collisions
        for transform in crabs_query.iter() {
            break;
        }

        // Barrier collisions
        for transform in barriers_query.iter() {
            break;
        }

        // Wall collisions
        for (wall_transform, goal) in walls_query.iter() {
            let wall_position = wall_transform.translation;
            let (n, distance_to_goal) = match goal {
                Goal::Top => (-Vec3::Z, radius_position.z - wall_position.z),
                Goal::Right => (Vec3::X, -radius_position.x + wall_position.x),
                Goal::Bottom => (Vec3::Z, -radius_position.z + wall_position.z),
                Goal::Left => (-Vec3::X, radius_position.x - wall_position.x),
            };

            // Slight offset from the wall so the ball doesn't phase into it.
            // Also prevents it from being treated as out of bounds.
            if distance_to_goal > 0.01 {
                continue;
            }

            let r = (d - (2.0 * (d.dot(n) * n))).normalize();
            commands
                .entity(entity)
                .insert(Velocity(r * config.ball_speed));
            break;
        }
    }
}

fn goal_scored_system(
    mut commands: Commands,
    config: Res<GameConfig>,
    mut game: ResMut<Game>,
    mut goal_eliminated_writer: EventWriter<GoalEliminated>,
    balls_query: Query<
        (Entity, &GlobalTransform, &Velocity),
        (With<Ball>, With<Active>, Without<Fade>),
    >,
    walls_query: Query<(&GlobalTransform, &Goal), With<Wall>>,
) {
    for (entity, ball_transform, velocity) in balls_query.iter() {
        let ball_translation = ball_transform.translation;
        let ball_radius = config.ball_radius();
        let d = velocity.0.normalize();
        let radius_position = ball_translation + d * ball_radius;

        for (wall_transform, goal) in walls_query.iter() {
            // Score against the goal that's closest to this ball
            let goal_position = wall_transform.translation;
            let distance_to_goal = match goal {
                Goal::Top => radius_position.z - goal_position.z,
                Goal::Right => -radius_position.x + goal_position.x,
                Goal::Bottom => -radius_position.z + goal_position.z,
                Goal::Left => radius_position.x - goal_position.x,
            };

            if distance_to_goal > 0.0 {
                continue;
            }

            // TODO: This would be a good opportunity for a score event!

            // Decrement the score and potentially eliminate the goal
            let score = game.scores.get_mut(goal).unwrap();
            *score = score.saturating_sub(1);

            if *score == 0 {
                goal_eliminated_writer.send(GoalEliminated(*goal))
            }

            // Fade out and deactivate the ball to prevent repeated scoring
            commands.entity(entity).insert(Fade::Out(0.0));
            break;
        }
    }
}

fn gameover_check_system(
    mut game: ResMut<Game>,
    mut state: ResMut<State<GameState>>,
    mut goal_eliminated_reader: EventReader<GoalEliminated>,
    players_query: Query<&Goal, (With<Crab>, With<Player>)>,
    enemies_query: Query<&Goal, (With<Crab>, With<Enemy>)>,
) {
    for GoalEliminated(_) in goal_eliminated_reader.iter() {
        // Player wins if all Enemy crabs have a score of zero
        let has_player_won =
            enemies_query.iter().all(|goal| game.scores[&goal] == 0);

        // Player loses if all Player crabs have a score of zero
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

// TODO: Need to document the hell out of this code.

// TODO: Need a fix for the rare occasion when a ball just bounces infinitely
// between two walls in a straight line?

// TODO: Offer a "Traditional" mode with two crabs (1xPlayer, 1xEnemy) opposite
// each other and the other two walled off. Also just one ball?

// TODO: Debug option to make all crabs driven by AI? Will need to revise
// player system to handle no players.

// TODO: Debug option to directly control single ball's exact position with
// keyboard and see how crabs respond. Can go in goals, triggering a score and
// ball return?

// TODO: Debug option to add small cubes at the projected points on goals with
// debug lines to the nearest ball. Also add a line from the crab to a flat
// but wide cube (to allow both to be visible if they overlap) that matches the
// crab's hit box dimensions and is positioned where the crab predicts it
// will stop. One of each per goal so we can spawn them in advance.
