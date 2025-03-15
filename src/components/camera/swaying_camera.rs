use bevy::prelude::*;

use crate::game::state::LoadedSet;

pub(super) struct SwayingCameraPlugin;

impl Plugin for SwayingCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            make_camera_slowly_sway_back_and_forth.in_set(LoadedSet),
        );
    }
}

/// Marks a [`Camera3d`] entity to sway back and forth in a slow reciprocating
/// motion while focusing on a central point.
#[derive(Component, Debug)]
#[require(Camera3d, Transform)]
pub struct SwayingCamera {
    pub target: Vec3,
    pub starting_position: Vec3,
    pub up_direction: Vec3,
    pub range: f32,
    pub speed: f32,
}

fn make_camera_slowly_sway_back_and_forth(
    time: Res<Time>,
    mut query: Query<(&SwayingCamera, &mut Transform), With<Camera3d>>,
) {
    let (swaying_camera, mut transform) = query.single_mut();
    let x_offset = (time.elapsed_secs() * swaying_camera.speed).sin()
        * (0.5 * swaying_camera.range);
    let mut new_position = swaying_camera.starting_position.clone();

    new_position.x += x_offset;

    *transform = Transform::from_translation(new_position)
        .looking_at(swaying_camera.target, swaying_camera.up_direction);
}
