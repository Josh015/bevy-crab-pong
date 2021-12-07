use super::*;
use crate::GameConfig;
use bevy::{
    core::Time,
    math::Vec3,
    prelude::{Component, Query, Res, Transform, With},
    render::camera::PerspectiveProjection,
};

/// A component that causes a camera to sway back and forth in a slow
/// reciprocating motion as it focuses on the origin.
#[derive(Component, Default)]
pub struct SwayingCamera {
    angle: f32,
}

/// Makes a `SwayingCamera` entity slowly sway back and forth.
pub fn swaying_system(
    config: Res<GameConfig>,
    time: Res<Time>,
    mut query: Query<
        (&mut Transform, &mut SwayingCamera),
        With<PerspectiveProjection>,
    >,
) {
    let (mut transform, mut swaying_camera) = query.single_mut();
    let x = swaying_camera.angle.sin() * ARENA_HALF_WIDTH;

    *transform = Transform::from_xyz(x, 2.0, 1.5)
        .looking_at(*ARENA_CENTER_POINT, Vec3::Y);

    swaying_camera.angle += config.swaying_camera_speed * time.delta_seconds();
    swaying_camera.angle %= std::f32::consts::TAU;
}
