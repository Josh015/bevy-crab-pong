use crate::prelude::*;

/// A component that causes a camera to sway back and forth in a slow
/// reciprocating motion as it focuses on the origin.
#[derive(Component)]
pub struct SwayingCamera;

/// Makes a [`SwayingCamera`] and [`Camera3d`] entity slowly sway back and
/// forth.
pub fn swaying_camera(
    config: Res<GameConfig>,
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<SwayingCamera>, With<Camera3d>)>,
) {
    let mut transform = query.single_mut();
    let x = (time.elapsed_seconds() * config.swaying_camera_speed).sin()
        * GOAL_HALF_WIDTH;

    *transform = Transform::from_xyz(x * 0.5, 2.0, 1.5)
        .looking_at(ARENA_CENTER_POINT, Vec3::Y);
}

pub struct SwayingCameraPlugin;

impl Plugin for SwayingCameraPlugin {
    fn build(&self, app: &mut App) { app.add_system(swaying_camera); }
}
