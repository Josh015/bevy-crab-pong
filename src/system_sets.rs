use bevy::prelude::*;

use super::GameState;

pub(super) struct SystemSetsPlugin;

impl Plugin for SystemSetsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            LoadedSet.run_if(not(in_state(GameState::Loading))),
        )
        .configure_sets(
            Update,
            PausableSet
                .in_set(LoadedSet)
                .run_if(not(in_state(GameState::Paused))),
        )
        .configure_sets(
            Update,
            PlayableSet
                .in_set(LoadedSet)
                .after(PausableSet)
                .run_if(in_state(GameState::Playing)),
        )
        .configure_sets(
            PostUpdate,
            LoadedSet.run_if(not(in_state(GameState::Loading))),
        )
        .configure_sets(
            PostUpdate,
            PausableSet
                .in_set(LoadedSet)
                .run_if(not(in_state(GameState::Paused))),
        )
        .configure_sets(
            PostUpdate,
            PlayableSet
                .in_set(LoadedSet)
                .after(PausableSet)
                .run_if(in_state(GameState::Playing)),
        );
    }
}

/// Systems that are always running after everything is loaded.
#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub struct LoadedSet;

/// Systems that stop when the game is paused.
#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub struct PausableSet;

/// Systems that only run during gameplay.
#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub struct PlayableSet;
