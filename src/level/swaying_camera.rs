use bevy::prelude::*;

use crate::{
    game::state::LoadedSet,
    level::{beach::BEACH_CENTER_POINT, goal::GOAL_WIDTH},
};

/// Marks a [`Camera3d`] entity to sway back and forth in a slow reciprocating
/// motion while looking at the center of the beach.
#[derive(Component, Debug)]
pub struct SwayingCamera {
    pub speed: f32,
}
pub(super) struct SwayingCameraPlugin;

impl Plugin for SwayingCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            make_camera_slowly_sway_back_and_forth.in_set(LoadedSet),
        );
    }
}

fn make_camera_slowly_sway_back_and_forth(
    time: Res<Time>,
    mut query: Query<(&SwayingCamera, &mut Transform), With<Camera3d>>,
) {
    let (swaying_camera, mut transform) = query.single_mut();
    let x = (time.elapsed_seconds() * swaying_camera.speed).sin()
        * (0.5 * GOAL_WIDTH);

    *transform = Transform::from_xyz(x * 0.5, 2.0, 1.5)
        .looking_at(BEACH_CENTER_POINT, Vec3::Y);
}
