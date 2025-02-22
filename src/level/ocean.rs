use bevy::prelude::*;

use crate::game::state::LoadedSet;

/// Marks an entity as an ocean with an animated texture effect.
#[derive(Clone, Component, Debug, Default)]
#[require(Transform, Visibility)]
pub struct Ocean {
    pub speed: f32,
}

pub(super) struct OceanPlugin;

impl Plugin for OceanPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            animate_ocean_with_scrolling_texture_effect.in_set(LoadedSet),
        );
    }
}

fn animate_ocean_with_scrolling_texture_effect(
    time: Res<Time>,
    mut scroll: Local<f32>,
    mut query: Query<(&Ocean, &mut Transform)>,
) {
    // HACK: Translate the plane on the Z-axis, since we currently can't
    // animate the texture coordinates.
    let (ocean, mut transform) = query.single_mut();

    *transform = Transform::from_xyz(0.0, -0.01, -*scroll);
    *scroll += ocean.speed * time.delta_secs();
    *scroll %= 1.0;
}
