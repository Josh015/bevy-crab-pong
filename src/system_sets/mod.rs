mod collisions;
mod debugging;
mod despawning;
mod effects;
mod gameplay_logic;
mod movement;
mod startup;
mod user_interface;

use crate::{global_data::GlobalData, screens::GameScreen};
use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameSystemSet {
    Collisions,
    Debugging,
    Despawning,
    Effects,
    GameplayLogic,
    Movement,
    UserInterface,
}

fn show_debugging_gizmos(global_data: Res<GlobalData>) -> bool {
    global_data.is_debugging_enabled
}

pub struct SystemSetsPlugin;

impl Plugin for SystemSetsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(Update, GameSystemSet::UserInterface)
            .configure_set(Update, GameSystemSet::Effects)
            .configure_set(
                Update,
                GameSystemSet::GameplayLogic
                    .before(GameSystemSet::Movement)
                    .run_if(in_state(GameScreen::Playing)),
            )
            .configure_set(
                Update,
                GameSystemSet::Movement
                    .after(GameSystemSet::Effects)
                    .run_if(not(in_state(GameScreen::Paused))),
            )
            .configure_set(
                PostUpdate,
                GameSystemSet::Collisions
                    .after(GameSystemSet::Movement)
                    .run_if(not(in_state(GameScreen::Paused))),
            )
            .configure_set(
                PostUpdate,
                GameSystemSet::Debugging
                    .after(GameSystemSet::Collisions)
                    .before(GameSystemSet::Despawning)
                    .run_if(show_debugging_gizmos)
                    .run_if(not(in_state(GameScreen::StartMenu))),
            )
            .configure_set(
                PostUpdate,
                GameSystemSet::Despawning
                    .after(GameSystemSet::Collisions)
                    .run_if(not(in_state(GameScreen::Paused))),
            )
            .add_plugins((
                collisions::CollisionsPlugin,
                debugging::DebuggingPlugin,
                despawning::DespawningPlugin,
                effects::EffectsPlugin,
                gameplay_logic::GameplayLogicPlugin,
                movement::MovementPlugin,
                startup::StartupPlugin,
                user_interface::UserInterfacePlugin,
            ));
    }
}
