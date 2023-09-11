use bevy::prelude::*;

use crate::components::spawning::Despawning;

use super::GameSystemSet;

fn finish_despawning_entity(
    mut commands: Commands,
    mut removed: RemovedComponents<Despawning>,
) {
    for removed_entity in removed.iter() {
        commands.entity(removed_entity).despawn_recursive();
    }
}

pub struct DespawningPlugin;

impl Plugin for DespawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            finish_despawning_entity.in_set(GameSystemSet::Despawning),
        );
    }
}
