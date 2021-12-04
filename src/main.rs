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
        .add_system(fade_start_system)
        .add_system(fade_step_system)
        .add_system(crab_fade_animation_system)
        .add_system(wall_fade_start_system)
        .add_system(wall_fade_animation_system)
        .add_system(ball_fade_animation_system)
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
                .with_system(crab_movement_system)
                .with_system(crab_player_control_system)
                .with_system(crab_ai_control_system)
                .with_system(ball_movement_system)
                .with_system(ball_reset_position_system)
                .with_system(ball_reset_velocity_system)
                .with_system(ball_collision_system)
                .with_system(goal_scored_system)
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
    scores: HashMap<Goal, u32>,
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

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum Movement {
    Stopped,
    Left,
    Right,
}

impl Default for Movement {
    fn default() -> Self { Self::Stopped }
}

struct GoalEliminated(Goal);

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
enum Goal {
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
struct Wall;

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

fn display_scores_system(
    game: Res<Game>,
    mut query: Query<(&mut Text, &Goal), With<Score>>,
) {
    for (mut text, goal) in query.iter_mut() {
        let score_value = game.scores[&goal];
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

fn fade_start_system(
    mut commands: Commands,
    query: Query<(Entity, &Fade), Added<Fade>>,
) {
    for (entity, fade) in query.iter() {
        if matches!(*fade, Fade::Out(_)) {
            commands.entity(entity).remove::<Active>();
        }
    }
}

fn fade_step_system(
    mut commands: Commands,
    config: Res<GameConfig>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Fade)>,
) {
    // Simulates fade from visible->invisible and vice versa over time
    for (entity, mut fade) in query.iter_mut() {
        let step = config.fade_speed * time.delta_seconds();

        match *fade {
            Fade::In(weight) => {
                if weight < 1.0 {
                    *fade = Fade::In(weight.max(0.0) + step);
                } else {
                    commands.entity(entity).remove::<Fade>().insert(Active);
                }
            },
            Fade::Out(weight) => {
                if weight < 1.0 {
                    *fade = Fade::Out(weight.max(0.0) + step);
                } else {
                    commands.entity(entity).remove::<Fade>();
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

fn wall_fade_start_system(
    mut commands: Commands,
    query: Query<(Entity, &Fade), (With<Wall>, Added<Fade>)>,
) {
    for (entity, fade) in query.iter() {
        // Immediately activate walls to avoid repeating eliminated animation
        if matches!(*fade, Fade::In(_)) {
            commands.entity(entity).insert(Active);
        }
    }
}

fn wall_fade_animation_system(
    config: Res<GameConfig>,
    mut query: Query<(&mut Transform, &Fade), With<Wall>>,
) {
    // Wall shrinks along its width into a pancake and then vanishes
    for (mut transform, fade) in query.iter_mut() {
        let x_mask = fade.opacity();
        let yz_mask = x_mask.powf(0.001);

        transform.scale =
            config.wall_scale() * Vec3::new(x_mask, yz_mask, yz_mask);
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
    let mut is_prior_fading = false;

    for (material, mut transform, mut fade) in query.iter_mut() {
        let is_current_fading = matches!(*fade, Fade::In(_));

        // Force current ball to wait if other is also fading in
        if is_prior_fading && is_current_fading {
            *fade = Fade::In(0.0);
            continue;
        }

        is_prior_fading = is_current_fading;

        // materials
        //     .get_mut(material)
        //     .unwrap()
        //     .base_color
        //     .set_a(fade.opacity());

        // FIXME: Use scaling until we can get alpha-blending working
        transform.scale = Vec3::splat(fade.opacity() * config.ball_size);
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
        (&Transform, &GlobalTransform, &Goal, &mut Crab),
        (With<Active>, With<Enemy>),
    >,
) {
    for (local, global, goal, mut crab) in crabs_query.iter_mut() {
        // Pick which ball is closest to this crab's goal
        let mut closest_ball_distance = std::f32::MAX;
        let mut target_position = config.crab_start_position.0;

        for ball_transform in balls_query.iter() {
            // Remap from ball's global space to crab's local space
            let ball_translation = ball_transform.translation;
            let ball_distance = global.translation.distance(ball_translation);
            let ball_position = match *goal {
                Goal::Top => -ball_translation.x,
                Goal::Right => -ball_translation.z,
                Goal::Bottom => ball_translation.x,
                Goal::Left => ball_translation.z,
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

fn ball_reset_velocity_system(
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

fn ball_collision_system(
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

// TODO: Need to split this file up into proper modules now that the divisions
// mostly seem to have settled.

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
