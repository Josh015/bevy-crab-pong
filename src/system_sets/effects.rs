use bevy::prelude::*;

use crate::{
    components::effects::*, constants::*, serialization::Config,
    system_sets::GameSystemSet,
};

fn make_camera_slowly_sway_back_and_forth(
    config: Res<Config>,
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<SwayingCamera>, With<Camera3d>)>,
) {
    let mut transform = query.single_mut();
    let x = (time.elapsed_seconds() * config.swaying_camera_speed).sin()
        * GOAL_HALF_WIDTH;

    *transform = Transform::from_xyz(x * 0.5, 2.0, 1.5)
        .looking_at(FIELD_CENTER_POINT, Vec3::Y);
}

fn animate_ocean_with_scrolling_texture_effect(
    config: Res<Config>,
    time: Res<Time>,
    mut query: Query<(&mut Ocean, &mut Transform)>,
) {
    // FIXME: Translate the plane on the Z-axis, since we currently can't
    // animate the texture coordinates.
    let (mut animated_water, mut transform) = query.single_mut();

    *transform = Transform::from_xyz(0.0, -0.01, animated_water.scroll);

    animated_water.scroll += config.animated_water_speed * time.delta_seconds();
    animated_water.scroll %= 1.0;
}

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                make_camera_slowly_sway_back_and_forth,
                animate_ocean_with_scrolling_texture_effect,
            )
                .in_set(GameSystemSet::Effects),
        );
    }
}
