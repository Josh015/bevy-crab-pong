use bevy::{math::Affine2, prelude::*, utils::HashMap};
use bevy_ui_anchor::{
    AnchorTarget, AnchorUiNode, HorizontalAnchor, VerticalAnchor,
};
use rand::prelude::*;
use strum::IntoEnumIterator;

use crate::{
    assets::{CachedAssets, CrabController, GameAssets, GameConfig},
    components::{
        AI, Acceleration, Ball, CircleCollider, Collider, Crab, CrabCollider,
        DepthCollider, Direction, Fade, FadeEffect, ForStates, Goal, GoalMouth,
        HitPoints, HitPointsUi, InsertAfterFadeIn, MaxSpeed, Motion, Player,
        Pole, RemoveBeforeFadeOut, ScrollingTexture, Side, Speed,
        SwayingCamera, Team, UiCamera,
    },
    states::GameState,
    system_params::GameModes,
    system_sets::ActiveDuringGameplaySet,
};

pub const LEVEL_CENTER_POINT: Vec3 = Vec3::ZERO;
pub const GOAL_ENTITY_LOCAL_START_POSITION: Vec3 = Vec3::ZERO;

pub(super) struct SpawnersPlugin;

impl Plugin for SpawnersPlugin {
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
            spawn_balls_sequentially_up_to_max_count
                .in_set(ActiveDuringGameplaySet),
        )
        .add_observer(spawn_pole_in_a_goal)
        .add_observer(spawn_ui_message);
    }
}

#[derive(Debug, Resource)]
pub struct Beach {
    pub width: f32,
}

/// An event fired to spawn a [`Pole`] in a [`Goal`].
#[derive(Debug, Event)]
pub struct SpawnPole {
    pub goal_entity: Entity,
    pub fade_in: bool,
}

/// An event fired when spawning a message UI.
#[derive(Debug, Event)]
pub struct SpawnUiMessage {
    pub message: String,
    pub game_state: GameState,
}

fn spawn_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_assets: Res<GameAssets>,
    game_configs: Res<Assets<GameConfig>>,
) {
    let game_config = game_configs.get(&game_assets.game_config).unwrap();

    commands.insert_resource(Beach {
        width: game_config.beach_width,
    });

    // Cameras
    commands.spawn((
        SwayingCamera {
            target: LEVEL_CENTER_POINT,
            starting_position: Vec3::new(0., 2., 1.5),
            up_direction: Vec3::Y,
            range: game_config.beach_width * 0.5,
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
        Mesh3d(meshes.add(Plane3d::default().mesh().size(2.0, 2.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 1.0, 1.0, 0.9),
            base_color_texture: Some(game_assets.image_water.clone()),
            alpha_mode: AlphaMode::Blend,
            // perceptual_roughness: 0.0,
            // reflectance: 1.0,
            uv_transform: Affine2::from_scale(Vec2::new(10., 10.)),
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
            Vec3::splat(game_config.beach_width),
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
            LEVEL_CENTER_POINT.with_z(0.5 * game_config.beach_width),
        ));

        let goal_entity = commands
            .spawn((
                Goal,
                GoalMouth {
                    width: game_config.beach_width
                        - game_config.barrier_diameter,
                },
                Team::default(),
                HitPoints::default(),
                side,
                goal_transform,
            ))
            .id();

        // Pole
        commands.trigger(SpawnPole {
            goal_entity,
            fade_in: false,
        });

        // HP
        let (offset, anchorwidth, anchorheight) = hp_ui_configs[&side];

        commands.spawn((
            HitPointsUi { goal_entity },
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

        // Corner Barriers
        commands.spawn((
            Collider,
            CircleCollider {
                radius: 0.5 * game_config.barrier_diameter,
            },
            Mesh3d(cylinder.clone()),
            MeshMaterial3d(barrier_material.clone()),
            goal_transform.mul_transform(Transform::from_matrix(
                Mat4::from_scale_rotation_translation(
                    Vec3::new(
                        game_config.barrier_diameter,
                        game_config.barrier_height,
                        game_config.barrier_diameter,
                    ),
                    Quat::IDENTITY,
                    Vec3::new(
                        0.5 * game_config.beach_width,
                        0.5 * game_config.barrier_height,
                        0.0,
                    ),
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
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cached_assets: Res<CachedAssets>,
    game_modes: GameModes,
    goals_query: Query<(Entity, &Side, Option<&Children>), With<Goal>>,
    game_assets: Res<GameAssets>,
    game_configs: Res<Assets<GameConfig>>,
) {
    let game_config = game_configs.get(&game_assets.game_config).unwrap();

    for (goal_entity, side, children) in &goals_query {
        if let Some(children) = children {
            for child in children {
                commands.entity(*child).insert(Fade::new_out());
            }
        }

        let crab_config = &game_modes.current().competitors[side];

        let id = commands
            .entity(goal_entity)
            .with_children(|builder| {
                let mut crab = builder.spawn((
                    Crab,
                    Collider,
                    CrabCollider {
                        width: game_config.crab_width,
                    },
                    DepthCollider {
                        depth: game_config.crab_depth,
                    },
                    InsertAfterFadeIn::<Motion>::default(),
                    RemoveBeforeFadeOut::<Motion>::default(),
                    RemoveBeforeFadeOut::<Collider>::default(),
                    Fade::new_in(),
                    FadeEffect::Scale {
                        max_scale: Vec3::new(
                            game_config.crab_width,
                            game_config.crab_depth,
                            game_config.crab_depth,
                        ),
                        axis_mask: Vec3::ONE,
                    },
                    Direction(Dir3::X),
                    MaxSpeed(crab_config.max_speed),
                    Acceleration(
                        crab_config.max_speed
                            / crab_config.seconds_to_max_speed,
                    ),
                    Mesh3d(cached_assets.crab_mesh.clone()),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(
                            game_assets.image_crab.clone(),
                        ),
                        base_color:
                            Srgba::hex(&crab_config.color).unwrap().into(),
                        ..default()
                    })),
                    Transform::from_matrix(
                        Mat4::from_scale_rotation_translation(
                            Vec3::splat(f32::EPSILON),
                            Quat::IDENTITY,
                            GOAL_ENTITY_LOCAL_START_POSITION
                                .with_y(game_config.crab_height_from_ground),
                        ),
                    ),
                ));

                if crab_config.controller == CrabController::AI {
                    crab.insert(AI);
                } else {
                    crab.insert(Player);
                }

                // TODO: Remove.
                crab.insert(*side);
            })
            .id();

        info!("Crab({id:?}): Spawned");
    }
}

fn spawn_balls_sequentially_up_to_max_count(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cached_assets: Res<CachedAssets>,
    game_modes: GameModes,
    non_moving_balls_query: Query<Entity, (With<Ball>, Without<Motion>)>,
    balls_query: Query<Entity, With<Ball>>,
    game_assets: Res<GameAssets>,
    game_configs: Res<Assets<GameConfig>>,
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
    let game_config = game_configs.get(&game_assets.game_config).unwrap();
    let mut rng = SmallRng::from_os_rng();
    let angle = rng.random_range(0.0..std::f32::consts::TAU);
    let (angle_sin, angle_cos) = angle.sin_cos();

    let ball = commands
        .spawn((
            Ball,
            CircleCollider {
                radius: game_mode.ball_scale * game_config.ball_diameter * 0.5,
            },
            Fade::new_in(),
            InsertAfterFadeIn::<Motion>::default(),
            InsertAfterFadeIn::<Collider>::default(),
            RemoveBeforeFadeOut::<Collider>::default(),
            ForStates(vec![GameState::Playing, GameState::Paused]),
            Direction(Dir3::new_unchecked(Vec3::new(
                angle_cos, 0.0, angle_sin,
            ))),
            Speed(game_mode.ball_speed),
            Mesh3d(cached_assets.ball_mesh.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                alpha_mode: AlphaMode::Blend,
                base_color: Color::srgba(1.0, 1.0, 1.0, 0.0),
                ..default()
            })),
            Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::splat(game_mode.ball_scale * game_config.ball_diameter),
                Quat::IDENTITY,
                LEVEL_CENTER_POINT.with_y(
                    game_mode.ball_scale * game_config.ball_height_from_ground,
                ),
            )),
        ))
        .id();

    info!("Ball({ball:?}): Spawned");
}

fn spawn_pole_in_a_goal(
    trigger: Trigger<SpawnPole>,
    cached_assets: Res<CachedAssets>,
    mut commands: Commands,
    goals_query: Query<Option<&Children>, With<Goal>>,
    game_assets: Res<GameAssets>,
    game_configs: Res<Assets<GameConfig>>,
) {
    let SpawnPole {
        goal_entity,
        fade_in,
    } = trigger.event();

    if let Ok(Some(children)) = goals_query.get(*goal_entity) {
        for child in children {
            commands.entity(*child).insert(Fade::new_out());
        }
    }

    let game_config = game_configs.get(&game_assets.game_config).unwrap();
    let id = commands
        .entity(*goal_entity)
        .with_children(|builder| {
            builder.spawn((
                Pole,
                Collider,
                DepthCollider {
                    depth: game_config.pole_diameter,
                },
                RemoveBeforeFadeOut::<Collider>::default(),
                if *fade_in {
                    Fade::new_in()
                } else {
                    Fade::In(Timer::default()) // Skip to end of animation.
                },
                FadeEffect::Scale {
                    max_scale: Vec3::new(
                        game_config.pole_diameter,
                        game_config.beach_width,
                        game_config.pole_diameter,
                    ),
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
                    GOAL_ENTITY_LOCAL_START_POSITION
                        .with_y(game_config.pole_height_from_ground),
                )),
            ));
        })
        .id();

    info!("Pole({id:?}): Spawned");
}

fn spawn_ui_message(
    trigger: Trigger<SpawnUiMessage>,
    game_assets: Res<GameAssets>,
    mut commands: Commands,
) {
    let SpawnUiMessage {
        message,
        game_state,
    } = trigger.event();

    commands.spawn((
        ForStates(vec![*game_state]),
        AnchorUiNode {
            target: AnchorTarget::Translation(LEVEL_CENTER_POINT),
            offset: None,
            anchorwidth: HorizontalAnchor::Mid,
            anchorheight: VerticalAnchor::Mid,
        },
        Text(message.clone()),
        TextFont {
            font: game_assets.font_menu.clone(),
            font_size: 25.0,
            ..default()
        },
        TextColor(Srgba::RED.into()),
    ));
}
