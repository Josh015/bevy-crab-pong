use bevy::{math::Affine2, prelude::*};
use strum::IntoEnumIterator;

use crate::{
    components::{
        ball::BallSpawner,
        collider::{CircleCollider, Collider},
        pole::SpawnPole,
        scrolling_texture::ScrollingTexture,
        side::{SIDE_WIDTH, Side, SideSpawnPoint},
        swaying_camera::SwayingCamera,
    },
    game::{
        assets::{GameAssets, GameConfig},
        state::GameState,
    },
};

pub const LEVEL_CENTER_POINT: Vec3 = Vec3::ZERO;
pub const BARRIER_DIAMETER: f32 = 0.12;
pub const BARRIER_RADIUS: f32 = 0.5 * BARRIER_DIAMETER;
pub const BARRIER_HEIGHT: f32 = 0.2;

pub(super) struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Loading), spawn_level);
    }
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
            speed: game_config.swaying_camera_speed,
            target: LEVEL_CENTER_POINT,
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
        BallSpawner::default(),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(1.0, 1.0))),
        MeshMaterial3d(materials.add(game_assets.image_sand.clone())),
        Transform::from_matrix(Mat4::from_scale_rotation_translation(
            Vec3::splat(SIDE_WIDTH),
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

    for (i, side) in Side::iter().enumerate() {
        // Spawn Point
        commands
            .spawn((
                SideSpawnPoint,
                side,
                Transform::from_rotation(Quat::from_axis_angle(
                    Vec3::Y,
                    std::f32::consts::TAU
                        * (i as f32 / Side::iter().len() as f32),
                ))
                .mul_transform(Transform::from_xyz(
                    0.0,
                    0.0,
                    0.5 * SIDE_WIDTH,
                )),
            ))
            .with_children(|builder| {
                // Barrier
                builder.spawn((
                    Collider,
                    CircleCollider {
                        radius: BARRIER_RADIUS,
                    },
                    Mesh3d(cylinder.clone()),
                    MeshMaterial3d(barrier_material.clone()),
                    Transform::from_matrix(
                        Mat4::from_scale_rotation_translation(
                            Vec3::new(
                                BARRIER_DIAMETER,
                                BARRIER_HEIGHT,
                                BARRIER_DIAMETER,
                            ),
                            Quat::IDENTITY,
                            Vec3::new(
                                0.5 * SIDE_WIDTH,
                                0.5 * BARRIER_HEIGHT,
                                0.0,
                            ),
                        ),
                    ),
                ));
            });

        // Poles
        commands.trigger(SpawnPole {
            side,
            fade_in: false,
        });
    }
}
