use bevy::{ecs::query::Has, prelude::*};
use strum::{EnumIter, IntoEnumIterator};

use crate::common::fade::{Fade, FadeAnimation};

/// Tags an entity to only exist in the listed game states.
#[derive(Clone, Component, Debug)]
pub struct ForStates<S: States>(pub Vec<S>);

// All the app's possible states.
#[derive(
    Clone, Copy, Debug, Default, Eq, EnumIter, Hash, PartialEq, States,
)]
pub enum GameState {
    #[default]
    Loading,
    StartMenu,
    Playing,
    Paused,
}

/// Runs after everything has finished loading.
#[derive(SystemSet, Clone, Hash, Debug, PartialEq, Eq)]
pub struct LoadedSet;

pub(super) struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().configure_sets(
            Update,
            LoadedSet.run_if(not(in_state(GameState::Loading))),
        );

        for state in GameState::iter() {
            app.add_systems(
                OnEnter(state),
                despawn_invalid_entities_for_state::<GameState>,
            );
        }
    }
}

fn despawn_invalid_entities_for_state<S: States>(
    mut commands: Commands,
    game_state: Res<State<S>>,
    query: Query<(Entity, &ForStates<S>, Has<FadeAnimation>)>,
) {
    for (entity, for_states, has_fade_animation) in &query {
        if !for_states.0.contains(game_state.get()) {
            if has_fade_animation {
                commands.entity(entity).insert(Fade::out_default());
            } else {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
