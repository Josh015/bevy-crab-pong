use bevy::prelude::*;

use crate::state::GameState;

/// Marks an entity as an ocean with an animated texture effect.
#[derive(Clone, Component, Debug, Default)]
pub struct Ocean {
    pub scroll: f32,
    pub speed: f32,
}

pub struct OceanPlugin;

impl Plugin for OceanPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            animate_ocean_with_scrolling_texture_effect
                .run_if(not(in_state(GameState::Loading))),
        );
    }
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
