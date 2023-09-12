mod collisions;
mod debugging;
mod despawning;
mod effects;
mod environment;
mod gameplay_logic;
mod movement;
mod spawning;
mod user_interface;

use bevy::prelude::*;
use spew::prelude::SpewSystemSet;

use crate::{resources::global_data::IsDebuggingMode, states::GameState};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub(super) enum GameSystemSet {
    Collisions,
    Debugging,
    Despawning,
    Effects,
    Environment,
    GameplayLogic,
    Movement,
    UserInterface,
}

fn show_debugging_gizmos(is_debugging_mode: Res<IsDebuggingMode>) -> bool {
    is_debugging_mode.0
}

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            Update,
            GameSystemSet::UserInterface
                .run_if(not(in_state(GameState::Loading))),
        )
        .configure_set(
            Update,
            GameSystemSet::Environment
                .run_if(not(in_state(GameState::Loading))),
        )
        .configure_set(
            Update,
            GameSystemSet::GameplayLogic
                .before(SpewSystemSet)
                .run_if(in_state(GameState::Playing)),
        )
        .configure_set(
            Update,
            GameSystemSet::Movement
                .after(GameSystemSet::Environment)
                .run_if(not(in_state(GameState::Loading)))
                .run_if(not(in_state(GameState::Paused))),
        )
        .configure_set(
            PostUpdate,
            GameSystemSet::Collisions
                .after(GameSystemSet::Movement)
                .run_if(not(in_state(GameState::Loading)))
                .run_if(not(in_state(GameState::Paused))),
        )
        .configure_set(
            PostUpdate,
            GameSystemSet::Effects
                .after(GameSystemSet::Collisions)
                .run_if(not(in_state(GameState::Loading)))
                .run_if(not(in_state(GameState::Paused))),
        )
        .configure_set(
            PostUpdate,
            GameSystemSet::Debugging
                .after(GameSystemSet::Effects)
                .before(GameSystemSet::Despawning)
                .run_if(show_debugging_gizmos)
                .run_if(not(in_state(GameState::Loading)))
                .run_if(not(in_state(GameState::StartMenu))),
        )
        .configure_set(
            PostUpdate,
            GameSystemSet::Despawning
                .after(GameSystemSet::Collisions)
                .run_if(not(in_state(GameState::Loading)))
                .run_if(not(in_state(GameState::Paused))),
        )
        .add_plugins((
            collisions::CollisionsPlugin,
            debugging::DebuggingPlugin,
            despawning::DespawningPlugin,
            effects::EffectsPlugin,
            environment::EnvironmentPlugin,
            gameplay_logic::GameplayLogicPlugin,
            movement::MovementPlugin,
            spawning::SpawningPlugin,
            user_interface::UserInterfacePlugin,
        ));
    }
}
