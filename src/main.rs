mod files;

use bevy::{
    ecs::prelude::*, prelude::*, render::camera::PerspectiveProjection,
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
        .add_system(display_scores_system)
        .add_system(swaying_camera_system)
        .add_system(animated_water_system)
        .add_system(fading_system)
        .add_system(crab_fading_animation_system)
        .add_system(pole_fading_animation_system)
        .add_system(ball_fading_animation_system)
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
                .with_system(reset_game_entities),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Playing).with_system(fade_out_balls),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(crab_movement_system)
                .with_system(crab_player_control_system)
                .with_system(crab_ai_control_system)
                .with_system(ball_movement_system)
                .with_system(ball_collision_system)
                .with_system(goal_scored_system)
                .with_system(goal_eliminated_animation_system)
                .with_system(gameover_check_system),
        )
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Playing,
    GameOver,
}

#[derive(Default)]
struct Game {
    scores: HashMap<GoalSide, u32>,
    winner: Option<Pilot>,
}

#[derive(Debug, Deserialize)]
struct GameConfig {
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
    ball_speed: f32,
    barrier_width: f32,
    fading_speed: f32,
    pole_radius: f32,
    starting_score: u32,
}

impl GameConfig {
    fn crab_acceleration(&self) -> f32 {
        self.crab_max_speed / self.crab_seconds_to_max_speed
    }

    fn ball_radius(&self) -> f32 { 0.5 * self.ball_size }

    fn pole_scale(&self) -> Vec3 {
        Vec3::new(self.beach_width, self.pole_radius, self.pole_radius)
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum Movement {
    Stopped,
    Left,
    Right,
}

impl Default for Movement {
    fn default() -> Self { Self::Stopped }
}

struct GoalEliminated(GoalSide);

#[derive(Component, Default)]
struct SwayingCamera {
    angle: f32,
}

#[derive(Component)]
struct Score;

#[derive(Component, Default)]
struct AnimatedWater {
    scroll: f32,
}

#[derive(Component, Default)]
struct Crab {
    movement: Movement,
    speed: f32,
}

#[derive(Clone, Component, Copy, Eq, PartialEq, Debug, Hash)]
enum GoalSide {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Clone, Component, Copy, Eq, PartialEq, Debug, Hash)]
enum Pilot {
    Player,
    Ai,
}

impl Default for Pilot {
    fn default() -> Self { Self::Player }
}

#[derive(Component)]
struct Ball {
    direction: Vec3,
}

impl Default for Ball {
    fn default() -> Self { Self { direction: Vec3::X } }
}

#[derive(Component)]
struct Pole;

#[derive(Component)]
struct Goal;

#[derive(Component)]
enum Collider {
    Line { width: f32 },
    Circle { radius: f32 },
}

#[derive(Component)]
struct Active;

#[derive(Clone, Component, Copy, PartialEq, Debug)]
enum Fading {
    Out(f32),
    In(f32),
}

impl Fading {
    fn opacity(&self) -> f32 {
        match self {
            Self::In(weight) => *weight,
            Self::Out(weight) => 1.0 - weight,
        }
    }
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
                        Vec3::new(0.0, 0.1, 0.0),
                    ),
                ),
                ..Default::default()
            })
            .insert(Ball::default())
            .insert(Fading::Out(0.99))
            .insert(Collider::Circle {
                radius: config.ball_radius(),
            });
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
    let pole_material = materials.add(Color::hex("00A400").unwrap().into());
    let barrier_material = materials.add(Color::hex("750000").unwrap().into());
    let goal_configs = [
        (
            Pilot::Player,
            Color::RED,
            GoalSide::Bottom,
            Rect {
                bottom: Val::Px(5.0),
                right: Val::Px(5.0),
                ..Default::default()
            },
        ),
        (
            Pilot::Ai,
            Color::BLUE,
            GoalSide::Right,
            Rect {
                top: Val::Px(5.0),
                right: Val::Px(5.0),
                ..Default::default()
            },
        ),
        (
            Pilot::Ai,
            Color::ORANGE,
            GoalSide::Top,
            Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
        ),
        (
            Pilot::Ai,
            Color::PURPLE,
            GoalSide::Left,
            Rect {
                bottom: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
        ),
    ];

    for (i, (pilot, color, goal_side, rect)) in goal_configs.iter().enumerate()
    {
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
            .insert(Goal)
            .insert(goal_side.clone())
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
                    .insert(Crab::default())
                    .insert(Fading::Out(0.99))
                    .insert(pilot.clone())
                    .insert(Collider::Line {
                        width: config.crab_scale.0,
                    })
                    .insert(goal_side.clone());

                // Pole
                parent
                    .spawn_bundle(PbrBundle {
                        mesh: unit_cube.clone(),
                        material: pole_material.clone(),
                        transform: Transform::from_matrix(
                            Mat4::from_scale_rotation_translation(
                                config.pole_scale(),
                                Quat::IDENTITY,
                                Vec3::new(0.0, 0.1, 0.0),
                            ),
                        ),
                        ..Default::default()
                    })
                    .insert(Pole)
                    .insert(goal_side.clone())
                    .insert(Active)
                    .insert(Collider::Line {
                        width: config.beach_width,
                    });

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
                    .insert(Active)
                    .insert(Collider::Circle {
                        radius: 0.5 * config.barrier_width,
                    });
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
            .insert(Score)
            .insert(goal_side.clone());

        game.scores.insert(goal_side.clone(), 0);
    }
}

fn display_scores_system(
    game: Res<Game>,
    mut query: Query<(&mut Text, &GoalSide), With<Score>>,
) {
    for (mut text, goal_side) in query.iter_mut() {
        let score_value = game.scores[&goal_side];
        text.sections[0].value = score_value.to_string();
    }
}

fn swaying_camera_system(
    config: Res<GameConfig>,
    time: Res<Time>,
    mut query: Query<
        (&mut Transform, &mut SwayingCamera),
        With<PerspectiveProjection>,
    >,
) {
    // Slowly sway the camera back and forth
    let (mut transform, mut swaying_camera) = query.single_mut();
    let x = swaying_camera.angle.sin() * 0.5 * config.beach_width;

    *transform = Transform::from_xyz(x, 2.0, 1.5)
        .looking_at(config.beach_center_point.into(), Vec3::Y);

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

fn fading_system(
    mut commands: Commands,
    config: Res<GameConfig>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Fading)>,
) {
    // Simulates fading from visible->invisible and vice versa over time
    let step = config.fading_speed * time.delta_seconds();

    for (entity, mut fading) in query.iter_mut() {
        match *fading {
            Fading::In(weight) => {
                if weight < 1.0 {
                    *fading = Fading::In(weight.max(0.0) + step);
                } else {
                    commands.entity(entity).remove::<Fading>();
                    commands.entity(entity).insert(Active);
                }
            },
            Fading::Out(weight) => {
                if weight < 1.0 {
                    *fading = Fading::Out(weight.max(0.0) + step);
                } else {
                    commands.entity(entity).remove::<Fading>();
                    commands.entity(entity).remove::<Active>();
                }
            },
        }
    }
}

fn crab_fading_animation_system(
    config: Res<GameConfig>,
    mut query: Query<(&mut Transform, &Fading), With<Crab>>,
) {
    // Grow/Shrink crabs to show/hide them
    for (mut transform, fading) in query.iter_mut() {
        transform.scale = config.crab_scale.into();
        transform.scale *= fading.opacity();
    }
}

fn pole_fading_animation_system(
    config: Res<GameConfig>,
    mut query: Query<(&mut Transform, &Fading), With<Pole>>,
) {
    // Pole shrinks along its width into a pancake and then vanishes
    for (mut transform, fading) in query.iter_mut() {
        let x_mask = fading.opacity();
        let yz_mask = x_mask.powf(0.001);

        transform.scale =
            config.pole_scale() * Vec3::new(x_mask, yz_mask, yz_mask);
    }
}

fn ball_fading_animation_system(
    config: Res<GameConfig>,
    asset_server: Res<AssetServer>,
    mut query: Query<
        (&mut Handle<StandardMaterial>, &mut Transform, &mut Fading),
        With<Ball>,
    >,
) {
    // Increase/Decrease balls' opacity to show/hide them
    let mut is_prior_fading = false;

    for (mut material, mut transform, mut fading) in query.iter_mut() {
        let is_current_fading = matches!(*fading, Fading::In(_));

        // Force current ball to wait if other is also fading in
        if is_prior_fading && is_current_fading {
            *fading = Fading::In(0.0);
            continue;
        }

        is_prior_fading = is_current_fading;

        // FIXME: Use scaling until we can get opacity working.
        transform.scale = Vec3::splat(fading.opacity() * config.ball_size);

        // TODO: Reduce ball opacity
        // asset_server.get_mut(&material).unwrap();
        // material.base_color.a = fading.opacity();
    }
}

fn gameover_show_ui(game: Res<Game>) {
    if let Some(winner) = game.winner {
        if winner == Pilot::Player {
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

fn reset_game_entities(
    mut commands: Commands,
    config: Res<GameConfig>,
    mut game: ResMut<Game>,
    mut crabs_query: Query<
        (Entity, &mut Transform),
        (With<Crab>, Without<Active>),
    >,
    poles_query: Query<Entity, (With<Pole>, With<Active>)>,
) {
    // Reset crabs
    for (entity, mut transform) in crabs_query.iter_mut() {
        commands.entity(entity).insert(Fading::In(0.4));
        transform.translation = config.crab_start_position.into();
    }

    // Reset poles
    for entity in poles_query.iter() {
        commands.entity(entity).insert(Fading::Out(0.3));
    }

    // Reset scores
    for (_, score) in game.scores.iter_mut() {
        *score = config.starting_score;
    }
}

fn fade_out_balls(
    mut commands: Commands,
    query: Query<(Entity, Option<&Fading>, Option<&Active>), With<Ball>>,
) {
    for (entity, fading, active) in query.iter() {
        match fading {
            Some(Fading::In(weight)) => {
                commands.entity(entity).insert(Fading::Out(1.0 - weight));
            },
            None if active.is_some() => {
                commands.entity(entity).insert(Fading::Out(0.0));
            },
            _ => {},
        }
    }
}

fn crab_movement_system(
    config: Res<GameConfig>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Crab), With<Active>>,
) {
    for (mut transform, mut crab) in query.iter_mut() {
        // Accelerate the crab
        let delta_speed = config.crab_acceleration() * time.delta_seconds();

        if crab.movement == Movement::Stopped {
            let s = crab.speed.abs().sub(delta_speed).max(0.0);
            crab.speed = crab.speed.max(-s).min(s);
        } else {
            crab.speed = crab
                .speed
                .add(if crab.movement == Movement::Left {
                    -delta_speed
                } else {
                    delta_speed
                })
                .clamp(-config.crab_max_speed, config.crab_max_speed);
        }

        // Limit crab to open space between barriers
        let mut position = transform.translation.x + crab.speed;
        let extents = 0.5
            * (config.beach_width - config.barrier_width - config.crab_scale.0);

        if position >= extents {
            position = extents;
            crab.speed = 0.0;
        } else if position <= -extents {
            position = -extents;
            crab.speed = 0.0;
        }

        // Move the crab
        transform.translation.x = position;
    }
}

fn crab_player_control_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Pilot, &mut Crab), With<Active>>,
) {
    for (pilot, mut crab) in query.iter_mut() {
        if *pilot != Pilot::Player {
            continue;
        }

        if keyboard_input.pressed(KeyCode::Left) {
            crab.movement = Movement::Left;
        } else if keyboard_input.pressed(KeyCode::Right) {
            crab.movement = Movement::Right;
        } else {
            crab.movement = Movement::Stopped;
        }
    }
}

// TODO: Try to implement with both GlobalTransforms so we can dump the touchy
// distance logic.
fn crab_ai_control_system(
    config: Res<GameConfig>,
    balls_query: Query<&GlobalTransform, (With<Ball>, With<Active>)>,
    mut crabs_query: Query<
        (&Transform, &Pilot, &GoalSide, &mut Crab),
        With<Active>,
    >,
) {
    for (crab_transform, pilot, goal_side, mut crab) in crabs_query.iter_mut() {
        if *pilot != Pilot::Ai {
            continue;
        }

        // Pick which ball is closest to this crab's goal
        let mut closest_ball_distance = std::f32::MAX;
        let mut target_position = config.crab_start_position.0;

        for ball_transform in balls_query.iter() {
            // Remap from ball's global space to crab's local space
            let ball_translation = ball_transform.translation;
            let ball_radius = config.ball_radius();
            let (ball_distance, ball_position) = match *goal_side {
                GoalSide::Top => {
                    (ball_translation.z - ball_radius, -ball_translation.x)
                },
                GoalSide::Right => {
                    (ball_translation.x - ball_radius, -ball_translation.z)
                },
                GoalSide::Bottom => {
                    (ball_translation.z + ball_radius, ball_translation.x)
                },
                GoalSide::Left => {
                    (ball_translation.x + ball_radius, ball_translation.z)
                },
            };

            if ball_distance < closest_ball_distance {
                target_position = ball_position;
                closest_ball_distance = ball_distance;
            }
        }

        // Predict the position where the crab will stop if it immediately
        // begins decelerating.
        let d = crab.speed * crab.speed / config.crab_acceleration();
        let stop_position = if crab.speed > 0.0 {
            crab_transform.translation.x + d
        } else {
            crab_transform.translation.x - d
        };

        // Begin decelerating if the ball will land over 70% of the crab's
        // middle at its predicted stop position. Otherwise go left/right
        // depending on which side of the crab it's approaching.
        if (stop_position - target_position).abs()
            < 0.7 * (config.crab_scale.0 * 0.5)
        {
            crab.movement = Movement::Stopped;
        } else if target_position < crab_transform.translation.x {
            crab.movement = Movement::Left;
        } else {
            crab.movement = Movement::Right;
        }
    }
}

fn ball_movement_system(
    mut commands: Commands,
    config: Res<GameConfig>,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut Transform,
        &mut Ball,
        Option<&Active>,
        Option<&Fading>,
    )>,
) {
    let mut rng = rand::thread_rng();

    for (entity, mut transform, mut ball, active, fading) in query.iter_mut() {
        // Move active balls in a straight path until they hit something
        if active.is_some() {
            transform.translation +=
                ball.direction * (config.ball_speed * time.delta_seconds());
        } else if fading.is_none() {
            // Move ball back to center
            transform.translation = config.beach_center_point.into();

            // Give the ball a random direction vector
            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
            ball.direction = Vec3::new(angle.cos(), 0.0, angle.sin());

            // Start fading it back into view
            commands.entity(entity).insert(Fading::In(0.0));
        }
    }
}

fn ball_collision_system(
    mut bally_query: Query<
        (Entity, &GlobalTransform, &Collider, &mut Ball),
        With<Active>,
    >,
    colliders_query: Query<(Entity, &GlobalTransform, &Collider), With<Active>>,
) {
    for (entity, transform, collider, mut ball) in bally_query.iter_mut() {
        // Colliders
        for (entity2, transform2, collider2) in colliders_query.iter() {
            // Collide with active entities that aren't the current one
            if entity != entity2 {
                // TODO: Run collision logic
                match collider2 {
                    Collider::Circle { radius } => {
                        // TODO: Circle-Circle collision
                        // How to detect and handle the other ball?
                    },
                    Collider::Line { width } => {
                        // TODO: Circle-Rectangle collision
                    },
                }
            }
        }
    }
}

fn goal_scored_system(
    mut commands: Commands,
    config: Res<GameConfig>,
    mut game: ResMut<Game>,
    mut goal_eliminated_writer: EventWriter<GoalEliminated>,
    balls_query: Query<(Entity, &GlobalTransform), (With<Ball>, With<Active>)>,
    goals_query: Query<(&GlobalTransform, &GoalSide), With<Goal>>,
) {
    for (entity, ball_transform) in balls_query.iter() {
        let distance_to_center = ball_transform
            .translation
            .distance(config.beach_center_point.into());
        let beach_widths = Vec2::splat(config.beach_width);
        let beach_radius = 0.5 * beach_widths.dot(beach_widths).sqrt();

        // Check if a ball has gone out of bounds
        if distance_to_center >= beach_radius {
            let mut closest_distance = std::f32::MAX;
            let mut scored_goal = GoalSide::Bottom;

            // Score against the goal that's closest to this ball
            for (goal_transform, goal_side) in goals_query.iter() {
                let new_distance = ball_transform
                    .translation
                    .distance(goal_transform.translation);

                if new_distance < closest_distance {
                    closest_distance = new_distance;
                    scored_goal = goal_side.clone();
                }
            }

            // Decrement the score and potentially eliminate the goal
            let score = game.scores.get_mut(&scored_goal).unwrap();
            *score = score.saturating_sub(1);

            if *score == 0 {
                goal_eliminated_writer.send(GoalEliminated(scored_goal))
            }

            // Trigger ball return and prevent repeated scoring
            commands.entity(entity).remove::<Active>();
            commands.entity(entity).insert(Fading::Out(0.0));
        }
    }
}

fn goal_eliminated_animation_system(
    mut commands: Commands,
    mut goal_eliminated_reader: EventReader<GoalEliminated>,
    balls_query: Query<(Entity, &GoalSide), (With<Crab>, With<Active>)>,
    poles_query: Query<(Entity, &GoalSide), (With<Pole>, Without<Active>)>,
) {
    for GoalEliminated(eliminated_side) in goal_eliminated_reader.iter() {
        for (entity, goal_side) in balls_query.iter() {
            if goal_side == eliminated_side {
                commands.entity(entity).remove::<Active>();
                commands.entity(entity).insert(Fading::Out(0.0));
                break;
            }
        }

        for (entity, goal_side) in poles_query.iter() {
            if goal_side == eliminated_side {
                commands.entity(entity).insert(Active);
                commands.entity(entity).insert(Fading::In(0.0));
                break;
            }
        }
    }
}

fn gameover_check_system(
    mut game: ResMut<Game>,
    mut state: ResMut<State<GameState>>,
    mut goal_eliminated_reader: EventReader<GoalEliminated>,
    query: Query<(&Pilot, &GoalSide), (With<Crab>, With<Active>)>,
) {
    for GoalEliminated(_) in goal_eliminated_reader.iter() {
        let has_player_won = query.iter().all(|(pilot, goal_side)| {
            *pilot != Pilot::Ai || game.scores[&goal_side] == 0
        });

        // Player loses if all Player crabs have a score of zero
        let has_player_lost = query.iter().all(|(pilot, goal_side)| {
            *pilot != Pilot::Player || game.scores[&goal_side] == 0
        });

        // Declare a winner and trigger gameover
        if has_player_won || has_player_lost {
            game.winner = if has_player_won {
                Some(Pilot::Player)
            } else {
                Some(Pilot::Ai)
            };

            state.set(GameState::GameOver).unwrap();
        }
    }
}

// TODO: Must do goal_eliminated_animation_system() before
// gameover_check_system() to ensure active crab check can happen without score
// check!

// TODO: Find a fix for balls/crabs that need to start with a scale of zero but
// can't because setting that as their initial scale causes them to stay hidden
// forever. Find something without resorting to a fade value of 0.9999.

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
