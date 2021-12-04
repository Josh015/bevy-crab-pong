use bevy::{
    core::Time,
    math::Vec3,
    prelude::{Component, Query, Res, Transform, With},
    render::camera::PerspectiveProjection,
};

use crate::GameConfig;

#[derive(Component, Default)]
pub struct SwayingCamera {
    angle: f32,
}

pub(super) fn swaying_system(
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
