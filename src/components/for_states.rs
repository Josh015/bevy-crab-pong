use bevy::prelude::*;
use strum::IntoEnumIterator;

use crate::prelude::*;

pub(super) struct ForStatesPlugin;

impl Plugin for ForStatesPlugin {
    fn build(&self, app: &mut App) {
        for state in GameState::iter() {
            app.add_systems(
                OnEnter(state),
                despawn_invalid_entities_for_state::<GameState>,
            );
        }
    }
}

/// Tags an entity to only exist in its associated game states.
#[derive(Clone, Component, Debug)]
pub struct ForStates<S: States>(pub Vec<S>);

fn despawn_invalid_entities_for_state<S: States>(
    mut commands: Commands,
    game_state: Res<State<S>>,
    query: Query<(Entity, &ForStates<S>, Has<FadeEffect>)>,
) {
    for (entity, for_states, has_fade_effect) in &query {
        if !for_states.0.contains(game_state.get()) {
            if has_fade_effect {
                commands.entity(entity).insert(Fade::new_out());
            } else {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
