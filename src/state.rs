use bevy::{ecs::query::Has, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

use crate::{
    assets::{GameAssets, GameConfig},
    common::fade::{Fade, FadeAnimation},
};

/// Tags an entity to only exist in the listed game states.
#[derive(Clone, Component, Debug)]
pub struct ForStates<S: States>(pub Vec<S>);

// All the app's possible states.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum GameState {
    #[default]
    Loading,
    StartMenu,
    Playing,
    Paused,
}

impl GameState {
    pub const ANY_GAME_STATE: &[GameState; 2] =
        &[GameState::Playing, GameState::Paused];

    pub fn is_any_game_state(&self) -> bool {
        GameState::ANY_GAME_STATE.contains(self)
    }
}

/// Runs after everything has finished loading.
#[derive(SystemSet, Clone, Hash, Debug, PartialEq, Eq)]
pub struct LoadedSet;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().add_plugins(RonAssetPlugin::<GameConfig>::new(&["config.ron"]))
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::StartMenu),
        )
        .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
            GameState::Loading,
            "game.assets.ron",
        )
        .add_collection_to_loading_state::<_, GameAssets>(GameState::Loading)
        .configure_set(
            Update,
            LoadedSet.run_if(not(in_state(GameState::Loading))),
        );

        for state in GameState::variants() {
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
