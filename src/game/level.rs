use bevy::{math::Affine2, prelude::*, utils::HashMap};
use bevy_ui_anchor::{
    AnchorTarget, AnchorUiNode, HorizontalAnchor, VerticalAnchor,
};
use rand::prelude::*;
use strum::IntoEnumIterator;

use crate::{
    components::{
        AI, Acceleration, Ball, CRAB_DEPTH, CRAB_WIDTH, CircleCollider,
        Collider, Crab, CrabCollider, Fade, FadeEffect, ForStates, Goal,
        Heading, HitPoints, InsertAfterFadeIn, MaxSpeed, Movement, Player,
        Pole, RemoveBeforeFadeOut, ScrollingTexture, Side, Speed,
        SwayingCamera, Team, UiCamera,
    },
    ui::HitPointsUiSource,
};

use super::{
    CachedAssets, CrabController, GameAssets, GameConfig, GameModes, GameState,
    PausableSet, PlayableSet, SpawnPole,
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
        app.add_systems(
            OnExit(GameState::Loading),
            (spawn_level, reset_team_and_hit_points).chain(),
        )
        .add_systems(
            OnExit(GameState::StartMenu),
            (spawn_crabs_for_each_side, reset_team_and_hit_points),
        )
        .add_systems(
            Update,
            spawn_balls_sequentially_up_to_max_count.in_set(PlayableSet),
        )
        .add_systems(
            Update,
            despawn_existing_crab_or_pole_per_side.in_set(PausableSet),
        )
        .insert_resource(Level { width: GOAL_WIDTH });
    }
}

#[derive(Debug, Resource)]
pub struct Level {
    pub width: f32,
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
        IsDefaultUiCamera,
        Msaa::Sample8,
        // Msaa::Off,
        // TemporalAntiAliasing::default(),
        // ScreenSpaceAmbientOcclusion {
        //     quality_level: ScreenSpaceAmbientOcclusionQualityLevel::High,
        //     ..default()
        // },
        // ScreenSpaceReflections::default(),
        UiCamera,
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
    let hp_ui_configs = HashMap::from([
        (
            Side::Bottom,
            (
                Some(0.25 * Vec3::Z),
                HorizontalAnchor::Right,
                VerticalAnchor::Bottom,
            ),
        ),
        (
            Side::Right,
            (
                Some(0.25 * Vec3::X),
                HorizontalAnchor::Right,
                VerticalAnchor::Bottom,
            ),
        ),
        (
            Side::Top,
            (
                Some(0.25 * Vec3::NEG_Z),
                HorizontalAnchor::Right,
                VerticalAnchor::Bottom,
            ),
        ),
        (
            Side::Left,
            (
                Some(0.25 * Vec3::NEG_X),
                HorizontalAnchor::Right,
                VerticalAnchor::Bottom,
            ),
        ),
    ]);

    for (i, side) in Side::iter().enumerate() {
        // Goal
        let goal_transform = Transform::from_rotation(Quat::from_axis_angle(
            Vec3::Y,
            std::f32::consts::TAU * (i as f32 / num_sides as f32),
        ))
        .mul_transform(Transform::from_translation(
            LEVEL_CENTER_POINT.with_z(0.5 * GOAL_WIDTH),
        ));

        let goal_entity = commands
            .spawn((
                Goal,
                Team::default(),
                HitPoints::default(),
                side,
                goal_transform,
            ))
            .id();

        let (offset, anchorwidth, anchorheight) = hp_ui_configs[&side];

        commands.spawn((
            HitPointsUiSource(goal_entity),
            AnchorUiNode {
                target: AnchorTarget::Entity(goal_entity),
                offset,
                anchorwidth,
                anchorheight,
            },
            Text("0".to_string()),
            TextFont {
                font: game_assets.font_menu.clone(),
                font_size: 50.0,
                ..Default::default()
            },
            TextColor(Srgba::RED.into()),
        ));

        // Pole
        commands.trigger(SpawnPole {
            goal_entity,
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

fn reset_team_and_hit_points(
    game_modes: GameModes,
    mut goals_query: Query<(&Side, &mut Team, &mut HitPoints)>,
) {
    for (side, competitor) in &game_modes.current().competitors {
        for (goal_side, mut team, mut hp) in &mut goals_query {
            if goal_side != side {
                continue;
            }

            team.0 = competitor.team.into();
            hp.0 = competitor.hit_points.into();
        }
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

fn despawn_existing_crab_or_pole_per_side(
    mut commands: Commands,
    new_query: Query<(Entity, &Parent), Or<(Added<Crab>, Added<Pole>)>>,
    old_query: Query<(Entity, &Parent), Or<(With<Crab>, With<Pole>)>>,
) {
    for (new_entity, new_parent) in &new_query {
        for (old_entity, old_parent) in &old_query {
            if old_parent == new_parent && old_entity != new_entity {
                commands.entity(old_entity).insert(Fade::new_out());
                break;
            }
        }
    }
}
