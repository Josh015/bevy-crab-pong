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
        .add_system(fade_system)
        .add_system(crab_fade_animation_system)
        .add_system(pole_fade_animation_system)
        .add_system(ball_fade_animation_system)
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
                .with_system(crab_movement_system)
                .with_system(crab_player_control_system)
                .with_system(crab_ai_control_system)
                .with_system(ball_movement_system)
                .with_system(ball_reset_position_system)
                .with_system(ball_reset_velocity_system)
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
    over: Option<GameOver>,
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
    fade_speed: f32,
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

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
enum GameOver {
    Won,
    Lost,
}

impl Default for GameOver {
    fn default() -> Self { Self::Won }
}

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Pole;

#[derive(Component)]
struct Goal;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Active;

#[derive(Clone, Component, Copy, PartialEq, Debug)]
enum Fade {
    Out(f32),
    In(f32),
}

impl Fade {
    fn opacity(&self) -> f32 {
        match self {
            Self::In(weight) => *weight,
            Self::Out(weight) => 1.0 - weight,
        }
    }
}

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Component)]
struct Barrier;

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
            .insert(Ball)
            .insert(Fade::Out(1.0));
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
            Color::RED,
            GoalSide::Bottom,
            Rect {
                bottom: Val::Px(5.0),
                right: Val::Px(5.0),
                ..Default::default()
            },
        ),
        (
            Color::BLUE,
            GoalSide::Right,
            Rect {
                top: Val::Px(5.0),
                right: Val::Px(5.0),
                ..Default::default()
            },
        ),
        (
            Color::ORANGE,
            GoalSide::Top,
            Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
        ),
        (
            Color::PURPLE,
            GoalSide::Left,
            Rect {
                bottom: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
        ),
    ];

    for (i, (color, goal_side, rect)) in goal_configs.iter().enumerate() {
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
                    .insert(Fade::Out(1.0))
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
                    .insert(Active);

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

fn fade_system(
    mut commands: Commands,
    config: Res<GameConfig>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Fade)>,
) {
    // Simulates fade from visible->invisible and vice versa over time
    let step = config.fade_speed * time.delta_seconds();

    for (entity, mut fade) in query.iter_mut() {
        match *fade {
            Fade::In(weight) => {
                if weight < 1.0 {
                    *fade = Fade::In(weight.max(0.0) + step);
                } else {
                    commands.entity(entity).remove::<Fade>();
                    commands.entity(entity).insert(Active);
                }
            },
            Fade::Out(weight) => {
                if weight < 1.0 {
                    *fade = Fade::Out(weight.max(0.0) + step);
                } else {
                    commands.entity(entity).remove::<Fade>();
                    commands.entity(entity).remove::<Active>();
                }
            },
        }
    }
}

fn crab_fade_animation_system(
    config: Res<GameConfig>,
    mut query: Query<(&mut Transform, &Fade), With<Crab>>,
) {
    // Grow/Shrink crabs to show/hide them
    for (mut transform, fade) in query.iter_mut() {
        transform.scale = config.crab_scale.into();
        transform.scale *= fade.opacity();
    }
}

fn pole_fade_animation_system(
    config: Res<GameConfig>,
    mut query: Query<(&mut Transform, &Fade), With<Pole>>,
) {
    // Pole shrinks along its width into a pancake and then vanishes
    for (mut transform, fade) in query.iter_mut() {
        let x_mask = fade.opacity();
        let yz_mask = x_mask.powf(0.001);

        transform.scale =
            config.pole_scale() * Vec3::new(x_mask, yz_mask, yz_mask);
    }
}

fn ball_fade_animation_system(
    config: Res<GameConfig>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<
        (&Handle<StandardMaterial>, &mut Transform, &mut Fade),
        With<Ball>,
    >,
) {
    // Increase/Decrease balls' opacity to show/hide them
    let mut is_prior_fade = false;

    for (material, mut transform, mut fade) in query.iter_mut() {
        let is_current_fade = matches!(*fade, Fade::In(_));

        // Force current ball to wait if other is also fade in
        if is_prior_fade && is_current_fade {
            *fade = Fade::In(0.0);
            continue;
        }

        is_prior_fade = is_current_fade;

        // materials
        //     .get_mut(material)
        //     .unwrap()
        //     .base_color
        //     .set_a(fade.opacity());

        // FIXME: Use scaling until we can get alpha-blending working
        transform.scale = Vec3::splat(fade.opacity() * config.ball_size);
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
    query: Query<(Entity, &GoalSide), With<Crab>>,
) {
    for (entity, goal_side) in query.iter() {
        if *goal_side == GoalSide::Bottom {
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
    poles_query: Query<Entity, (With<Pole>, With<Active>)>,
) {
    // Reset crabs
    for (entity, mut transform) in crabs_query.iter_mut() {
        commands.entity(entity).insert(Fade::In(0.4));
        transform.translation = config.crab_start_position.into();
    }

    // Reset poles
    for entity in poles_query.iter() {
        commands.entity(entity).insert(Fade::Out(0.3));
    }

    // Reset scores
    for (_, score) in game.scores.iter_mut() {
        *score = config.starting_score;
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
    mut query: Query<&mut Crab, (With<Active>, With<Player>)>,
) {
    for mut crab in query.iter_mut() {
        crab.movement = if keyboard_input.pressed(KeyCode::Left) {
            Movement::Left
        } else if keyboard_input.pressed(KeyCode::Right) {
            Movement::Right
        } else {
            Movement::Stopped
        };
    }
}

fn crab_ai_control_system(
    config: Res<GameConfig>,
    balls_query: Query<&GlobalTransform, (With<Ball>, With<Active>)>,
    mut crabs_query: Query<
        (&Transform, &GlobalTransform, &GoalSide, &mut Crab),
        (With<Active>, With<Enemy>),
    >,
) {
    for (local, global, goal_side, mut crab) in crabs_query.iter_mut() {
        // Pick which ball is closest to this crab's goal
        let mut closest_ball_distance = std::f32::MAX;
        let mut target_position = config.crab_start_position.0;

        for ball_transform in balls_query.iter() {
            // Remap from ball's global space to crab's local space
            let ball_translation = ball_transform.translation;
            let ball_distance = global.translation.distance(ball_translation);
            let ball_position = match *goal_side {
                GoalSide::Top => -ball_translation.x,
                GoalSide::Right => -ball_translation.z,
                GoalSide::Bottom => ball_translation.x,
                GoalSide::Left => ball_translation.z,
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
            local.translation.x + d
        } else {
            local.translation.x - d
        };

        // Begin decelerating if the ball will land over 70% of the crab's
        // middle at its predicted stop position. Otherwise go left/right
        // depending on which side of the crab it's approaching.
        if (stop_position - target_position).abs()
            < 0.7 * (config.crab_scale.0 * 0.5)
        {
            crab.movement = Movement::Stopped;
        } else if target_position < local.translation.x {
            crab.movement = Movement::Left;
        } else {
            crab.movement = Movement::Right;
        }
    }
}

fn ball_movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity), With<Ball>>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_seconds();
    }
}

fn ball_reset_position_system(
    mut commands: Commands,
    config: Res<GameConfig>,
    mut query: Query<
        (Entity, &mut Transform),
        (With<Ball>, Without<Active>, Without<Fade>),
    >,
) {
    for (entity, mut transform) in query.iter_mut() {
        transform.translation = config.beach_center_point.into();
        commands.entity(entity).remove::<Velocity>();
        commands.entity(entity).insert(Fade::In(0.0));
    }
}

fn ball_reset_velocity_system(
    mut commands: Commands,
    config: Res<GameConfig>,
    query: Query<Entity, (With<Ball>, With<Active>, Without<Velocity>)>,
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

fn ball_collision_system(
    config: Res<GameConfig>,
    balls_query: Query<
        (Entity, &GlobalTransform, &Velocity),
        (With<Ball>, With<Active>),
    >,
    crabs_query: Query<&GlobalTransform, (With<Crab>, With<Active>)>,
    poles_query: Query<&GoalSide, (With<Pole>, With<Active>)>,
    barriers_query: Query<&GlobalTransform, With<Barrier>>,
) {
    for (entity, transform, velocity) in balls_query.iter() {
        let ball_radius = config.ball_radius();
        let barrier_radius = 0.5 * config.barrier_width;

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

        // Pole collisions
        for goal_side in poles_query.iter() {
            break;
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
        // TODO: Cache some of these values via a resource?
        let beach_widths = Vec2::splat(config.beach_width);
        let beach_radius = 0.5 * beach_widths.dot(beach_widths).sqrt();
        let center: Vec3 = config.beach_center_point.into();
        let ball_translation = ball_transform.translation;
        let distance_to_center = ball_translation.distance(center);

        // Check if the ball has gone out of bounds
        if distance_to_center < beach_radius {
            continue;
        }

        // Score against the goal that's closest to this ball
        let mut closest_distance = std::f32::MAX;
        let mut scored_goal = GoalSide::Bottom;

        for (goal_transform, goal_side) in goals_query.iter() {
            let new_distance =
                ball_translation.distance(goal_transform.translation);

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

        // Fade out and deactivate the ball to prevent repeated scoring
        commands.entity(entity).remove::<Active>();
        commands.entity(entity).insert(Fade::Out(0.0));
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
                commands.entity(entity).insert(Fade::Out(0.0));
                break;
            }
        }

        for (entity, goal_side) in poles_query.iter() {
            if goal_side == eliminated_side {
                commands.entity(entity).insert(Active);
                commands.entity(entity).insert(Fade::In(0.0));
                break;
            }
        }
    }
}

fn gameover_check_system(
    mut game: ResMut<Game>,
    mut state: ResMut<State<GameState>>,
    mut goal_eliminated_reader: EventReader<GoalEliminated>,
    players_query: Query<&GoalSide, (With<Crab>, With<Player>)>,
    enemies_query: Query<&GoalSide, (With<Crab>, With<Enemy>)>,
) {
    for GoalEliminated(_) in goal_eliminated_reader.iter() {
        // Player wins if all Enemy crabs have a score of zero
        let has_player_won = enemies_query
            .iter()
            .all(|goal_side| game.scores[&goal_side] == 0);

        // Player loses if all Player crabs have a score of zero
        let has_player_lost = players_query
            .iter()
            .all(|goal_side| game.scores[&goal_side] == 0);

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
