use bevy::prelude::*;

use crate::{
    components::environment::{Ocean, SwayingCamera},
    constants::*,
};

use super::GameSystemSet;

fn make_camera_slowly_sway_back_and_forth(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &SwayingCamera), With<Camera3d>>,
) {
    let (mut transform, swaying_camera) = query.single_mut();
    let x =
        (time.elapsed_seconds() * swaying_camera.speed).sin() * GOAL_HALF_WIDTH;

    *transform = Transform::from_xyz(x * 0.5, 2.0, 1.5)
        .looking_at(FIELD_CENTER_POINT, Vec3::Y);
}

fn animate_ocean_with_scrolling_texture_effect(
    time: Res<Time>,
    mut query: Query<(&mut Ocean, &mut Transform)>,
) {
    // FIXME: Translate the plane on the Z-axis, since we currently can't
    // animate the texture coordinates.
    let (mut ocean, mut transform) = query.single_mut();

    *transform = Transform::from_xyz(0.0, -0.01, ocean.scroll);

    ocean.scroll += ocean.speed * time.delta_seconds();
    ocean.scroll %= 1.0;
}

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                make_camera_slowly_sway_back_and_forth,
                animate_ocean_with_scrolling_texture_effect,
            )
                .in_set(GameSystemSet::Environment),
        );
    }
}
