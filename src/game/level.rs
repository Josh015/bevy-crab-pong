use bevy::{math::Affine2, prelude::*};
use rand::prelude::*;
use strum::IntoEnumIterator;

use crate::{
    components::{
        ball::Ball,
        collider::{CircleCollider, Collider},
        crab::{
            CRAB_DEPTH, CRAB_WIDTH, Crab, CrabCollider, ai::AI, player::Player,
        },
        fade::{Fade, FadeEffect, InsertAfterFadeIn, RemoveBeforeFadeOut},
        goal::Goal,
        movement::{Acceleration, Heading, MaxSpeed, Movement, Speed},
        pole::{POLE_DIAMETER, POLE_HEIGHT, Pole},
        scrolling_texture::ScrollingTexture,
        side::Side,
        swaying_camera::SwayingCamera,
    },
    game::{
        assets::{CrabController, GameAssets, GameConfig},
        state::{ForStates, GameState},
    },
};

use super::{
    assets::CachedAssets,
    events::SideEliminatedEvent,
    state::{PausableSet, PlayableSet},
    system_params::GameModes,
};

pub const LEVEL_CENTER_POINT: Vec3 = Vec3::ZERO;
pub const BARRIER_DIAMETER: f32 = 0.12;
pub const BARRIER_RADIUS: f32 = 0.5 * BARRIER_DIAMETER;
pub const BARRIER_HEIGHT: f32 = 0.2;
pub const BALL_HEIGHT_FROM_GROUND: f32 = 0.05;
pub const GOAL_WIDTH: f32 = 1.0;
pub const CRAB_START_POSITION: Vec3 = Vec3::new(0.0, 0.05, 0.0);

pub(super) struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Level { width: GOAL_WIDTH })
            .add_observer(spawn_pole_on_a_side)
            .add_systems(OnExit(GameState::Loading), spawn_level)
            .add_systems(
                OnExit(GameState::StartMenu),
                spawn_crabs_for_each_side,
            )
            .add_systems(
                Update,
                spawn_balls_sequentially_up_to_max_count.in_set(PlayableSet),
            )
            .add_systems(
                PostUpdate,
                spawn_poles_for_eliminated_sides.after(PlayableSet),
            )
            .add_systems(
                Update,
                despawn_existing_crab_or_pole_per_side.in_set(PausableSet),
            );
    }
}

#[derive(Debug, Resource)]
pub struct Level {
    pub width: f32,
}

#[derive(Event)]
pub struct SpawnPole {
    pub side: Side,
    pub fade_in: bool,
}

fn spawn_pole_on_a_side(
    trigger: Trigger<SpawnPole>,
    cached_assets: Res<CachedAssets>,
    mut commands: Commands,
    goals_query: Query<(Entity, &Side), With<Goal>>,
) {
    let SpawnPole { side, fade_in } = trigger.event();
    let (goal_entity, _) = goals_query
        .iter()
        .find(|(_, goal_side)| **goal_side == *side)
        .unwrap();

    commands.entity(goal_entity).with_children(|builder| {
        builder.spawn((
            Pole,
            *side,
            Collider,
            RemoveBeforeFadeOut::<Collider>::default(),
            if *fade_in {
                Fade::new_in()
            } else {
                Fade::In(Timer::default()) // Skip to end of animation.
            },
            FadeEffect::Scale {
                max_scale: Vec3::new(POLE_DIAMETER, GOAL_WIDTH, POLE_DIAMETER),
                axis_mask: Vec3::new(1.0, 0.0, 1.0),
            },
            Mesh3d(cached_assets.pole_mesh.clone()),
            MeshMaterial3d(cached_assets.pole_material.clone()),
            Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::splat(f32::EPSILON),
                Quat::from_euler(
                    EulerRot::XYZ,
                    0.0,
                    0.0,
                    std::f32::consts::FRAC_PI_2,
                ),
                Vec3::new(0.0, POLE_HEIGHT, 0.0),
            )),
        ));
    });

    info!("Pole({side:?}): Spawned");
}

fn spawn_level(
    game_assets: Res<GameAssets>,
    game_configs: Res<Assets<GameConfig>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let game_config = game_configs.get(&game_assets.game_config).unwrap();

    // Cameras
    commands.spawn((
        SwayingCamera {
            target: LEVEL_CENTER_POINT,
            starting_position: Vec3::new(0., 2., 1.5),
            up_direction: Vec3::Y,
            range: GOAL_WIDTH * 0.5,
            speed: game_config.swaying_camera_speed,
        },
        Camera3d::default(),
        Msaa::Sample8,
        // Msaa::Off,
        // TemporalAntiAliasing::default(),
        // ScreenSpaceAmbientOcclusion {
        //     quality_level: ScreenSpaceAmbientOcclusionQualityLevel::High,
        //     ..default()
        // },
        // ScreenSpaceReflections::default(),
    ));

    // Light
    commands.spawn((
        DirectionalLight {
            illuminance: 2_500.0,
            // shadows_enabled: true,
            ..default()
        },
        Transform::from_matrix(Mat4::from_euler(
            EulerRot::ZYX,
            0.0,
            std::f32::consts::FRAC_PI_4,
            -std::f32::consts::FRAC_PI_4,
        )),
    ));

    // Ocean
    commands.spawn((
        ScrollingTexture {
            velocity: Vec2::Y * game_config.ocean_scroll_speed,
        },
        Mesh3d(meshes.add(Plane3d::default().mesh().size(1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 1.0, 1.0, 0.9),
            base_color_texture: Some(game_assets.image_water.clone()),
            alpha_mode: AlphaMode::Blend,
            // perceptual_roughness: 0.0,
            // reflectance: 1.0,
            uv_transform: Affine2::from_scale(Vec2::new(5., 5.)),
            ..default()
        })),
        Transform::from_matrix(Mat4::from_scale_rotation_translation(
            Vec3::new(5., 1., 5.),
            Quat::IDENTITY,
            Vec3::new(0., -0.01, 0.),
        )),
    ));

    // Beach
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(1.0, 1.0))),
        MeshMaterial3d(materials.add(game_assets.image_sand.clone())),
        Transform::from_matrix(Mat4::from_scale_rotation_translation(
            Vec3::splat(GOAL_WIDTH),
            Quat::IDENTITY,
            LEVEL_CENTER_POINT,
        )),
    ));

    // Goals
    let cylinder = meshes.add(Cylinder {
        half_height: 0.5,
        radius: 0.5,
    });
    let barrier_material =
        materials.add(Color::Srgba(Srgba::hex("750000").unwrap()));

    let num_sides = Side::iter().len();

    for (i, side) in Side::iter().enumerate() {
        // Goal
        let goal_transform = Transform::from_rotation(Quat::from_axis_angle(
            Vec3::Y,
            std::f32::consts::TAU * (i as f32 / num_sides as f32),
        ))
        .mul_transform(Transform::from_xyz(
            0.0,
            0.0,
            0.5 * GOAL_WIDTH,
        ));

        commands.spawn((Goal, side, goal_transform));

        // Pole
        commands.trigger(SpawnPole {
            side,
            fade_in: false,
        });

        // Corner Barriers
        commands.spawn((
            Collider,
            CircleCollider {
                radius: BARRIER_RADIUS,
            },
            Mesh3d(cylinder.clone()),
            MeshMaterial3d(barrier_material.clone()),
            goal_transform.mul_transform(Transform::from_matrix(
                Mat4::from_scale_rotation_translation(
                    Vec3::new(
                        BARRIER_DIAMETER,
                        BARRIER_HEIGHT,
                        BARRIER_DIAMETER,
                    ),
                    Quat::IDENTITY,
                    Vec3::new(0.5 * GOAL_WIDTH, 0.5 * BARRIER_HEIGHT, 0.0),
                ),
            )),
        ));
    }
}

fn spawn_crabs_for_each_side(
    cached_assets: Res<CachedAssets>,
    game_assets: Res<GameAssets>,
    game_modes: GameModes,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    goals_query: Query<(Entity, &Side), With<Goal>>,
) {
    for (goal_entity, side) in &goals_query {
        let crab_config = &game_modes.current().competitors[side];

        commands.entity(goal_entity).with_children(|builder| {
            let mut crab = builder.spawn((
                Crab,
                CrabCollider { width: CRAB_WIDTH },
                *side,
                Collider,
                InsertAfterFadeIn::<Movement>::default(),
                RemoveBeforeFadeOut::<Movement>::default(),
                RemoveBeforeFadeOut::<Collider>::default(),
                Fade::new_in(),
                FadeEffect::Scale {
                    max_scale: Vec3::new(CRAB_WIDTH, CRAB_DEPTH, CRAB_DEPTH),
                    axis_mask: Vec3::ONE,
                },
                Heading(Dir3::X),
                MaxSpeed(crab_config.max_speed),
                Acceleration(
                    crab_config.max_speed / crab_config.seconds_to_max_speed,
                ),
                Mesh3d(cached_assets.crab_mesh.clone()),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color_texture: Some(game_assets.image_crab.clone()),
                    base_color: Srgba::hex(&crab_config.color).unwrap().into(),
                    ..default()
                })),
                Transform::from_matrix(Mat4::from_scale_rotation_translation(
                    Vec3::splat(f32::EPSILON),
                    Quat::IDENTITY,
                    CRAB_START_POSITION,
                )),
            ));

            if crab_config.controller == CrabController::AI {
                crab.insert(AI);
            } else {
                crab.insert(Player);
            }
        });

        info!("Crab({side:?}): Spawned");
    }
}

fn spawn_balls_sequentially_up_to_max_count(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cached_assets: Res<CachedAssets>,
    game_modes: GameModes,
    non_moving_balls_query: Query<Entity, (With<Ball>, Without<Movement>)>,
    balls_query: Query<Entity, With<Ball>>,
) {
    // Wait for previously spawned ball to finish appearing.
    if non_moving_balls_query.iter().len() >= 1 {
        return;
    }

    // Spawn balls up to max ball count.
    let game_mode = game_modes.current();
    let ball_count: u8 = game_mode.ball_count.into();

    if balls_query.iter().len() >= ball_count as usize {
        return;
    }

    // Spawn a ball in a random direction from the center of the spawner.
    let mut rng = SmallRng::from_os_rng();
    let angle = rng.random_range(0.0..std::f32::consts::TAU);
    let (angle_sin, angle_cos) = angle.sin_cos();
    let position = LEVEL_CENTER_POINT.clone().with_y(BALL_HEIGHT_FROM_GROUND);

    let ball = commands
        .spawn((
            Ball,
            CircleCollider {
                radius: game_mode.ball_size * 0.5,
            },
            Fade::new_in(),
            InsertAfterFadeIn::<Movement>::default(),
            InsertAfterFadeIn::<Collider>::default(),
            RemoveBeforeFadeOut::<Collider>::default(),
            ForStates(vec![GameState::Playing, GameState::Paused]),
            Heading(Dir3::new_unchecked(Vec3::new(angle_cos, 0.0, angle_sin))),
            Speed(game_mode.ball_speed),
            Mesh3d(cached_assets.ball_mesh.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                alpha_mode: AlphaMode::Blend,
                base_color: Color::srgba(1.0, 1.0, 1.0, 0.0),
                ..default()
            })),
            Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::splat(game_mode.ball_size),
                Quat::IDENTITY,
                position,
            )),
        ))
        .id();

    info!("Ball({ball:?}): Spawned");
}

fn spawn_poles_for_eliminated_sides(
    mut side_eliminated_events: EventReader<SideEliminatedEvent>,
    mut commands: Commands,
) {
    for SideEliminatedEvent(side) in side_eliminated_events.read() {
        commands.trigger(SpawnPole {
            side: *side,
            fade_in: true,
        });
        info!("Side({side:?}): Eliminated");
    }
}

fn despawn_existing_crab_or_pole_per_side(
    mut commands: Commands,
    new_query: Query<(Entity, &Side), Or<(Added<Crab>, Added<Pole>)>>,
    old_query: Query<(Entity, &Side), Or<(With<Crab>, With<Pole>)>>,
) {
    for (new_entity, new_side) in &new_query {
        for (old_entity, old_side) in &old_query {
            if old_side == new_side && old_entity != new_entity {
                commands.entity(old_entity).insert(Fade::new_out());
                break;
            }
        }
    }
}
