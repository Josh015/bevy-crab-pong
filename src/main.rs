// TODO: New game screen: Scores visible and set to 0, barriers up, crabs
// hidden, balls hidden, new game text visible.

// TODO: Debug option to make all crabs driven by AI? Will need to revise player
// system to handle no players.

// TODO: Debug option to directly control single ball's exact position with
// keyboard and see how crabs respond. Can go in goals, triggering a score and
// ball return?

// TODO: Debug option to add small cubes at the projected points on goals with
// debug lines to the nearest ball. Also add a line from the crab to a flat but
// wide cube (to allow both to be visible if they overlap) that matches the
// crab's hit box dimensions and is positioned where the crab predicts it will
// stop. One of each per goal so we can spawn them in advance.

/*
Instead of handling different crabs individually, build Goal with child Barrier, Pole, and Crab together and have crab move on relative transform?

Don't de-spawn anything, just show/hide it based on the mode?

Need to have a flag and a float to handle fading/shrinking.

For crab, need to immediately set to inactive, halt AI, set walking to Stopped, and set fading to zero.

For ball, keep on trajectory until fully faded. Switching to active ball can't start moving until fully faded in.

Start with ball launch and return since we can just pick random directions, check if it's out of bounds, and then run return logic and keep re-launching it!

Need to trigger hide animations of remaining crabs and balls on win/lose event. Disable ball return as well.
*/

mod files;

use bevy::{ecs::prelude::*, prelude::*};
use rand::prelude::*;
use serde::Deserialize;
use std::{
    collections::HashMap,
    ops::{Add, Sub},
};

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

#[derive(Clone, /* Component, */ PartialEq, Debug)]
enum Visibility {
    Visible,
    FadingOut(f32),
    Invisible,
    FadingIn(f32),
}

impl Visibility {
    fn opacity(&self) -> f32 {
        match self {
            Visibility::Visible => 1.0,
            Visibility::Invisible => 0.0,
            Visibility::FadingIn(weight) => *weight,
            Visibility::FadingOut(weight) => 1.0 - weight,
        }
    }
}

enum CrabWalking {
    Stopped,
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
    walking: CrabWalking,
    /* speed0: f32,
     * pos0: f32, */
}

impl Default for Crab {
    fn default() -> Self {
        Self {
            goal_location: GoalLocation::Bottom,
            walking: CrabWalking::Stopped,
        }
    }
}

// #[derive(Component)]
struct Player;

// #[derive(Component)]
struct Opponent;

// #[derive(Component)]
struct Ball {
    direction: Vec3,
    radius: f32,
}

impl Default for Ball {
    fn default() -> Self {
        Self {
            direction: Vec3::ZERO,
            radius: 0.0,
        }
    }
}

// #[derive(Component)]
struct Pole {
    goal_location: GoalLocation,
    // is_active: bool,
}

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
    crab_max_scale: f32,
    crab_max_speed: f32,
    ball_speed: f32,
    fading_speed: f32,
    /* startingScore: u8, //20,
     * ballSpeed: f32,    // ?? */
}

struct Game {
    scores: HashMap<GoalLocation, u32>,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            scores: HashMap::from([
                (GoalLocation::Top, 0),
                (GoalLocation::Right, 0),
                (GoalLocation::Bottom, 0),
                (GoalLocation::Left, 0),
            ]),
        }
    }
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
        .insert_resource(Game::default())
        .add_startup_system(setup_level)
        .add_startup_system(setup_playable_entities)
        .add_system(swaying_camera_system)
        .add_system(animated_water_system)
        .add_system(display_scores_system)
        .add_system(visibility_lifecycle_system)
        .add_system(crab_visibility_system)
        .add_system(crab_walking_system)
        .add_system(player_crab_control_system)
        .add_system(ai_crab_control_system)
        .add_system(ball_visibility_system)
        .add_system(ball_collision_system)
        .add_system(ball_movement_system)
        .add_system(gameover_keyboard_system)
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
    config: Res<GameConfig>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Crabs
    let unit_cube = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let crab_scale = Vec3::splat(config.crab_max_scale);
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
            ..Default::default()
        })
        .insert(Visibility::FadingIn(0.0))
        .insert(Opponent)
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
            ..Default::default()
        })
        .insert(Visibility::FadingIn(0.0))
        .insert(Opponent)
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
            ..Default::default()
        })
        .insert(Visibility::FadingIn(0.0))
        .insert(Player)
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
            ..Default::default()
        })
        .insert(Visibility::FadingIn(0.0))
        .insert(Opponent)
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
        .insert(Ball::default())
        .insert(Visibility::Invisible)
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
        .insert(Ball::default())
        .insert(Visibility::Invisible)
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
        Transform::from_xyz(x, 2.25, 2.0).looking_at(Vec3::ZERO, Vec3::Y);

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

fn display_scores_system(
    game: Res<Game>,
    mut query: Query<(&mut Text, &Score)>,
) {
    for (mut text, score) in query.iter_mut() {
        let score_value = game.scores[&score.goal_location];
        text.sections[0].value = score_value.to_string();
    }
}

fn crab_walking_system(
    config: Res<GameConfig>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Crab, &Visibility)>,
) {
    for (mut transform, mut crab, visibility) in query.iter_mut() {
        if *visibility == Visibility::Visible {
            // TODO: This direction remapping can go away if we parent the crabs
            // and make it all relative!
            let left_direction = match crab.goal_location {
                GoalLocation::Top => Vec3::new(1.0, 0.0, 0.0),
                GoalLocation::Right => Vec3::new(0.0, 0.0, 1.0),
                GoalLocation::Bottom => Vec3::new(-1.0, 0.0, 0.0),
                GoalLocation::Left => Vec3::new(0.0, 0.0, -1.0),
            };
            let sign = match crab.walking {
                CrabWalking::Stopped => 0.0,
                CrabWalking::Left => 1.0,
                CrabWalking::Right => -1.0,
            };

            transform.translation +=
                sign * left_direction * time.delta_seconds();

            // TODO: speed0 is used for predicting stop position is AI.
            // TODO: pos0

            // // Accelerate the crab
            // const CRAB_STEP_TIME: f32 = 0.01;
            // const TIME_TO_MAXIMUM_SPEED: f32 = 0.18;
            // const CRAB_LENGTH: f32 = 0.2;
            // //The radius of the four barriers positioned at the corners
            // const BARRIER_SIZE: f32 = 0.12;

            // // TODO: This is used by multiple functions, but is not
            // crab-specific. let acceleration = config.crab_max_speed /
            // TIME_TO_MAXIMUM_SPEED;

            // let ds = CRAB_STEP_TIME * acceleration;

            // if sign != 0.0 {
            //     crab.speed0 = crab
            //         .speed0
            //         .add(sign * ds)
            //         .clamp(-config.crab_max_speed,
            // config.crab_max_speed); } else {
            //     let s = crab.speed0.abs().sub(ds).min(0.0);
            //     crab.speed0 = crab.speed0.min(-s).max(s); // Can't use
            // clamp() here. }

            // // Move the crab
            // crab.pos0 += CRAB_STEP_TIME * crab.speed0;

            // if crab.pos0 < BARRIER_SIZE + CRAB_LENGTH / 2.0 {
            //     crab.pos0 = BARRIER_SIZE + CRAB_LENGTH / 2.0;
            //     crab.speed0 = 0.0;
            // } else if crab.pos0 > 1.0 - BARRIER_SIZE - CRAB_LENGTH / 2.0 {
            //     crab.pos0 = 1.0 - BARRIER_SIZE - CRAB_LENGTH / 2.0;
            //     crab.speed0 = 0.0;
            // }

            // transform.translation =
            //     crab.pos0 * sign * left_direction * time.delta_seconds();
        }
    }
}

fn player_crab_control_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Crab, &Visibility), With<Player>>,
) {
    for (mut crab, visibility) in query.iter_mut() {
        if *visibility == Visibility::Visible {
            if keyboard_input.pressed(KeyCode::Left) {
                crab.walking = CrabWalking::Left;
            } else if keyboard_input.pressed(KeyCode::Right) {
                crab.walking = CrabWalking::Right;
            } else {
                crab.walking = CrabWalking::Stopped;
            }
        }
    }
}

fn ai_crab_control_system(
    balls_query: Query<&GlobalTransform, With<Ball>>,
    mut crab_query: Query<
        (&GlobalTransform, &mut Crab, &Visibility),
        With<Opponent>,
    >,
) {
    for (crab_transform, mut crab, visibility) in crab_query.iter_mut() {
        if *visibility == Visibility::Visible {
            // Pick which ball is closest to this crab's goal.
            for ball_transform in balls_query.iter() {
                // TODO:
                // Project ball center onto goal line.
                // Get normalized vector between ball center and projected
                // point. Multiply normal by radius, and offset
                // from ball center. Get `ball_distance` between
                // offset point and projected point.
                // Get `target_position` by figuring out which extent it is
                // closer to to find sign and make a weight
                // between the two points.
            }

            // Predict the crab's stop position if it begins decelerating.
            let stop_position = 0.0;

            // Begin decelerating if the ball will land over 70% of the crab's
            // middle at its predicted stop position. Otherwise go left/right
            // depending on which side of the crab it's approaching.
            if true {
                crab.walking = CrabWalking::Stopped;
            } else if false {
                crab.walking = CrabWalking::Left;
            } else {
                crab.walking = CrabWalking::Right;
            }
        }
    }
}

fn visibility_lifecycle_system(
    config: Res<GameConfig>,
    time: Res<Time>,
    mut query: Query<&mut Visibility>,
) {
    let step = config.fading_speed * time.delta_seconds();

    for mut visibility in query.iter_mut() {
        *visibility = match *visibility {
            Visibility::Visible => Visibility::Visible,
            Visibility::Invisible => Visibility::Invisible,
            Visibility::FadingIn(weight) => {
                if weight >= 1.0 {
                    Visibility::Visible
                } else {
                    Visibility::FadingIn(weight + step)
                }
            },
            Visibility::FadingOut(weight) => {
                if weight >= 1.0 {
                    Visibility::Invisible
                } else {
                    Visibility::FadingOut(weight + step)
                }
            },
        }
    }
}

fn crab_visibility_system(
    config: Res<GameConfig>,
    mut query: Query<(&mut Transform, &Visibility), With<Crab>>,
) {
    // Grow/Shrink crabs to show/hide them
    for (mut transform, visibility) in query.iter_mut() {
        transform.scale =
            Vec3::splat(visibility.opacity() * config.crab_max_scale);
    }
}

fn ball_visibility_system(
    config: Res<GameConfig>,
    asset_server: Res<AssetServer>,
    mut query: Query<
        (&mut Handle<StandardMaterial>, &mut Visibility),
        With<Ball>,
    >,
) {
    // Increase/Decrease balls' opacity to show/hide them
    let mut is_prior_fading = false;

    for (mut material, mut visibility) in query.iter_mut() {
        let is_current_fading = matches!(*visibility, Visibility::FadingIn(_));

        // Force current ball to wait if other is also fading in
        if is_prior_fading && is_current_fading {
            *visibility = Visibility::FadingIn(0.0);
        } else {
            is_prior_fading = is_current_fading;
            // TODO: Reduce ball opacity
            // asset_server.get_mut(&material).unwrap();
            // material.base_color.a = visibility.opacity();
        }
    }
}

fn ball_collision_system(
    mut query: Query<(&Transform, &mut Ball, &mut Visibility)>,
) {
    for (transform, mut ball, mut visibility) in query.iter_mut() {
        if *visibility == Visibility::Visible {
            // TODO: Run collision logic

            // Begin fading out ball when it scores
            if false {
                // TODO: Run scoring logic
                *visibility = Visibility::FadingOut(0.0);
            }
        }
    }
}

fn ball_movement_system(
    config: Res<GameConfig>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Ball, &mut Visibility)>,
) {
    let mut rng = rand::thread_rng();

    for (mut transform, mut ball, mut visibility) in query.iter_mut() {
        match *visibility {
            Visibility::Visible | Visibility::FadingOut(_) => {
                transform.translation +=
                    ball.direction * config.ball_speed * time.delta_seconds();
            },
            Visibility::Invisible => {
                // Move ball back to center, then start fading it into view
                *visibility = Visibility::FadingIn(0.0);
                transform.translation = Vec3::ZERO;

                // Give the ball a random direction vector
                let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                ball.direction.x = angle.cos();
                // ball.direction.y = 0.0;
                ball.direction.z = angle.sin();
            },
            _ => {},
        };
    }
}

fn display_gameover_screen(game: Res<Game>, query: Query<&Crab, With<Player>>) {
    // TODO: Gameover screen: Fade out balls. Fade out the last crab that
    // lost. Preserve crab(s) that didn't lose. Preserve scores. Disable AI,
    // collisions, ball return, etc.

    // Show win/lose text if there's a player and at least one non-zero score.
    if game.scores.iter().any(|score| score.1 > &0) {
        for crab in query.iter() {
            if game.scores[&crab.goal_location] > 0 {
                // If player score is non-zero, show win text.
            } else {
                // If player score is zero, show lose text.
            }
        }
    }

    // Show instructions for new game.
}

// TODO: Run for NewGame, Win, and Gameover
fn gameover_keyboard_system(keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        // TODO: ENTER starts new game.
    }
}
