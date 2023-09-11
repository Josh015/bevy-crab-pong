use bevy::prelude::*;

use crate::{
    components::spawning::{Despawning, ForStates, SpawnAnimation},
    screens::GameScreen,
};

use super::GameSystemSet;

fn despawn_invalid_entities_for_current_screen(
    mut commands: Commands,
    game_screen: Res<State<GameScreen>>,
    mut query: Query<
        (Entity, &ForStates<GameScreen>, Option<&SpawnAnimation>),
        Added<ForStates<GameScreen>>,
    >,
) {
    for (entity, for_states, spawning_animation) in &mut query {
        if !for_states.0.contains(game_screen.get()) {
            if spawning_animation.is_some() {
                commands.entity(entity).insert(Despawning);
            } else {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

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
            (
                despawn_invalid_entities_for_current_screen,
                finish_despawning_entity,
            )
                .chain()
                .in_set(GameSystemSet::Despawning),
        );
    }
}
