use bevy::prelude::*;

use crate::LoadedSet;

pub(super) struct ScrollingTexturePlugin;

impl Plugin for ScrollingTexturePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, scrolling_texture_effect.in_set(LoadedSet));
    }
}

/// Makes an entity auto-scroll its texture.
#[derive(Clone, Component, Debug, Default)]
#[require(MeshMaterial3d<StandardMaterial>)]
pub struct ScrollingTexture {
    pub velocity: Vec2,
}

fn scrolling_texture_effect(
    time: Res<Time>,
    query: Query<(&ScrollingTexture, &MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (scrolling_texture, mesh_material) in &query {
        let Some(material) = materials.get_mut(mesh_material.id()) else {
            continue;
        };

        material.uv_transform.translation =
            scrolling_texture.velocity * time.elapsed_secs();
    }
}
