use bevy::{ecs::query::Has, prelude::*};

use crate::spawning::{Despawning, SpawnAnimation};

/// Tags an entity to only exist in the listed game states.
#[derive(Clone, Component, Debug)]
pub struct ForStates<T: States, const N: usize>(pub [T; N]);

// All the app's possible states.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum AppState {
    #[default]
    Loading,
    StartMenu,
    Playing,
    Paused,
}

impl AppState {
    pub const ANY_GAME_STATE: &[AppState; 2] =
        &[AppState::Playing, AppState::Paused];

    pub fn is_any_game_state(&self) -> bool {
        AppState::ANY_GAME_STATE.contains(self)
    }
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>();

        for state in AppState::variants() {
            app.add_systems(
                OnEnter(state),
                (
                    despawn_invalid_entities_for_state::<AppState, 1>,
                    despawn_invalid_entities_for_state::<AppState, 2>,
                    despawn_invalid_entities_for_state::<AppState, 3>,
                ),
            );
        }
    }
}

fn despawn_invalid_entities_for_state<S: States, const N: usize>(
    mut commands: Commands,
    game_state: Res<State<S>>,
    query: Query<(Entity, &ForStates<S, N>, Has<SpawnAnimation>)>,
) {
    for (entity, for_states, has_spawning_animation) in &query {
        if !for_states.0.contains(game_state.get()) {
            if has_spawning_animation {
                commands.entity(entity).insert(Despawning);
            } else {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
