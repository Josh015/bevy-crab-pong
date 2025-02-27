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
    query: Query<(&Ocean, &MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let (ocean, mesh_material) = query.single();
    let Some(material) = materials.get_mut(mesh_material.id()) else {
        return;
    };

    material.uv_transform.translation.y = *scroll;
    *scroll += ocean.speed * time.delta_secs();
    *scroll %= 1.0;
}
