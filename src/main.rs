mod active;
mod animated_water;
mod ball;
mod barrier;
mod enemy;
mod fade;
mod files;
mod game;
mod gameover_message;
mod goal;
mod mirror;
mod movement;
mod paddle;
mod player;
mod score;
mod swaying_camera;
mod wall;

pub mod prelude {
    pub use crate::active::*;
    pub use crate::animated_water::*;
    pub use crate::ball::*;
    pub use crate::barrier::*;
    pub use crate::enemy::*;
    pub use crate::fade::*;
    pub use crate::game::*;
    pub use crate::gameover_message::*;
    pub use crate::goal::*;
    pub use crate::mirror::*;
    pub use crate::movement::*;
    pub use crate::paddle::*;
    pub use crate::player::*;
    pub use crate::score::*;
    pub use crate::swaying_camera::*;
    pub use crate::wall::*;
    pub use bevy::math::*;
    pub use bevy::prelude::*;
    pub use rand::prelude::*;
}

use crate::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Hash, SystemLabel)]
enum GameSystems {
    StatusChange,
    EntityLogic,
    Collisions,
    Transformations,
    Mirroring,
    GoalCheck,
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
        .insert_resource(ClearColor(config.clear_color))
        .add_plugins(DefaultPlugins)
        .init_resource::<Game>()
        .insert_resource(config)
        .add_state(GameState::GameOver)
        .add_event::<GoalEliminated>()
        .add_startup_system(setup)
        .add_system(score::update_scores_system)
        .add_system_set(
            SystemSet::new()
                .label(GameSystems::StatusChange)
                .with_system(fade::begin_fade_system)
                .with_system(wall::begin_fade_system),
        )
        .add_system_set(
            SystemSet::new()
                .label(GameSystems::Transformations)
                .after(GameSystems::Collisions)
                .with_system(animated_water::animation_system)
                .with_system(ball::fade_animation_system)
                .with_system(fade::step_fade_system)
                .with_system(paddle::fade_animation_system)
                .with_system(swaying_camera::swaying_system)
                .with_system(wall::fade_animation_system),
        )
        .add_system_set(
            SystemSet::new()
                .label(GameSystems::Mirroring)
                .after(GameSystems::Transformations)
                .with_system(mirror::reflect_parent_entities_system),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::GameOver)
                .with_system(game::show_gameover_ui),
        )
        .add_system_set(
            SystemSet::on_update(GameState::GameOver)
                .with_system(game::gameover_keyboard_system),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::GameOver)
                .with_system(game::hide_gameover_ui),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(game::reset_game_entities),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .label(GameSystems::EntityLogic)
                .after(GameSystems::StatusChange)
                .with_system(ball::inactive_ball_reset_system)
                .with_system(ball::reactivated_ball_launch_system)
                .with_system(enemy::ai_paddle_control_system)
                .with_system(player::keyboard_paddle_control_system),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .label(GameSystems::Collisions)
                .after(GameSystems::EntityLogic)
                .with_system(ball::collision_system)
                .with_system(ball::goal_scored_system)
                .with_system(paddle::bounded_movement_system)
                .with_system(paddle::stop_when_inactive_system),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .after(GameSystems::Transformations)
                .before(GameSystems::Mirroring)
                .with_system(movement::acceleration_system),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .label(GameSystems::GoalCheck)
                .after(GameSystems::Mirroring)
                .with_system(game::gameover_check_system)
                .with_system(goal::eliminated_system),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Playing)
                .with_system(game::fade_out_balls),
        )
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

/// Handles setting up all the entities that will be needed for every screen of
/// this game.
pub fn setup(
    config: Res<GameConfig>,
    mut game: ResMut<Game>,
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
    let light_transform = Mat4::from_euler(
        EulerRot::ZYX,
        0.0,
        std::f32::consts::FRAC_PI_4,
        -std::f32::consts::FRAC_PI_4,
    );
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            shadow_projection: OrthographicProjection {
                left: -10.0,
                right: 10.0,
                bottom: -10.0,
                top: 10.0,
                near: -50.0,
                far: 50.0,
                ..Default::default()
            },
            // shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_matrix(light_transform),
        ..Default::default()
    });

    // Ocean
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("257AFFCC").unwrap(),
                alpha_mode: AlphaMode::Blend,
                ..Default::default()
            }),
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
                ARENA_CENTER_POINT,
            ),
        ),
        ..Default::default()
    });

    // Balls
    let unit_sphere = meshes.add(Mesh::from(shape::Icosphere {
        radius: 0.5,
        subdivisions: 2,
    }));

    for _ in 0..2 {
        // Share material between original and mirrored, but not between balls
        // since we need them to have independent opacities.
        let material = materials.add(Color::WHITE.into());

        let mirrored_entity = commands
            .spawn_bundle(PbrBundle {
                mesh: unit_sphere.clone(),
                material: material.clone(),
                transform: Transform::from_matrix(
                    Mat4::from_scale_rotation_translation(
                        Vec3::splat(Ball::DIAMETER),
                        Quat::IDENTITY,
                        Ball::STARTING_POSITION,
                    ),
                ),
                ..Default::default()
            })
            .insert_bundle((
                Ball,
                Fade::Out(1.0),
                Movement {
                    direction: Vec3::ZERO,
                    speed: 0.0,
                    max_speed: config.ball_max_speed,
                    acceleration: config.ball_max_speed
                        / config.ball_seconds_to_max_speed,
                    delta: None,
                },
            ))
            .id();

        commands
            .spawn_bundle(PbrBundle {
                mesh: unit_sphere.clone(),
                material: material.clone(),
                transform: Transform::from_matrix(
                    Mat4::from_scale_rotation_translation(
                        Vec3::splat(Ball::DIAMETER),
                        Quat::IDENTITY,
                        Ball::STARTING_POSITION,
                    ),
                ),
                ..Default::default()
            })
            .insert(Mirror(mirrored_entity));
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
                let material = materials.add(color.clone().into());

                let mirrored_entity = parent
                    .spawn_bundle(PbrBundle {
                        mesh: unit_cube.clone(),
                        material: material.clone(),
                        transform: Transform::from_matrix(
                            Mat4::from_scale_rotation_translation(
                                Paddle::SCALE,
                                Quat::IDENTITY,
                                Paddle::START_POSITION,
                            ),
                        ),
                        ..Default::default()
                    })
                    .insert_bundle((
                        Paddle,
                        Fade::Out(1.0),
                        goal.clone(),
                        Movement {
                            direction: Vec3::X,
                            speed: 0.0,
                            max_speed: config.paddle_max_speed,
                            acceleration: config.paddle_max_speed
                                / config.paddle_seconds_to_max_speed,
                            delta: None,
                        },
                    ))
                    .id();

                parent
                    .spawn_bundle(PbrBundle {
                        mesh: unit_cube.clone(),
                        material: material.clone(),
                        transform: Transform::from_matrix(
                            Mat4::from_scale_rotation_translation(
                                Paddle::SCALE,
                                Quat::IDENTITY,
                                Paddle::START_POSITION,
                            ),
                        ),
                        ..Default::default()
                    })
                    .insert(Mirror(mirrored_entity));

                // Wall
                let mirrored_entity = parent
                    .spawn_bundle(PbrBundle {
                        mesh: unit_cube.clone(),
                        material: wall_material.clone(),
                        transform: Transform::from_matrix(
                            Mat4::from_scale_rotation_translation(
                                Wall::SCALE,
                                Quat::IDENTITY,
                                Vec3::new(0.0, Wall::HEIGHT, 0.0),
                            ),
                        ),
                        ..Default::default()
                    })
                    .insert_bundle((Wall, Active, goal.clone()))
                    .id();

                parent
                    .spawn_bundle(PbrBundle {
                        mesh: unit_cube.clone(),
                        material: wall_material.clone(),
                        transform: Transform::from_matrix(
                            Mat4::from_scale_rotation_translation(
                                Wall::SCALE,
                                Quat::IDENTITY,
                                Vec3::new(0.0, Wall::HEIGHT, 0.0),
                            ),
                        ),
                        ..Default::default()
                    })
                    .insert(Mirror(mirrored_entity));

                // Barrier
                let mirrored_entity = parent
                    .spawn_bundle(PbrBundle {
                        mesh: unit_cube.clone(),
                        material: barrier_material.clone(),
                        transform: Transform::from_matrix(
                            Mat4::from_scale_rotation_translation(
                                Vec3::new(
                                    Barrier::DIAMETER,
                                    Barrier::HEIGHT,
                                    Barrier::DIAMETER,
                                ),
                                Quat::IDENTITY,
                                Vec3::new(
                                    ARENA_HALF_WIDTH,
                                    0.5 * Barrier::HEIGHT,
                                    0.0,
                                ),
                            ),
                        ),
                        ..Default::default()
                    })
                    .insert(Barrier)
                    .id();

                parent
                    .spawn_bundle(PbrBundle {
                        mesh: unit_cube.clone(),
                        material: barrier_material.clone(),
                        transform: Transform::from_matrix(
                            Mat4::from_scale_rotation_translation(
                                Vec3::new(
                                    Barrier::DIAMETER,
                                    Barrier::HEIGHT,
                                    Barrier::DIAMETER,
                                ),
                                Quat::IDENTITY,
                                Vec3::new(
                                    ARENA_HALF_WIDTH,
                                    0.5 * Barrier::HEIGHT,
                                    0.0,
                                ),
                            ),
                        ),
                        ..Default::default()
                    })
                    .insert(Mirror(mirrored_entity));
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
            color: Color::NONE.into(),
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
                    color: Color::NONE.into(),
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
                                    font: font.clone(),
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

// TODO: Try to mimic the in-world text of the original with a RT texture?

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
