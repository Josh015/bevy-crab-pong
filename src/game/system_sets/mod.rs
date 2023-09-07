use crate::prelude::*;

mod collisions;
mod debugging;
mod despawning;
mod effects;
mod gameplay_logic;
mod movement;
mod startup;
mod user_interface;

pub use collisions::*;
pub use debugging::*;
pub use despawning::*;
pub use effects::*;
pub use gameplay_logic::*;
pub use movement::*;
pub use startup::*;
pub use user_interface::*;

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

fn show_debugging_gizmos(game_state: Res<GameState>) -> bool {
    game_state.is_debugging_enabled
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
                CollisionsPlugin,
                DebuggingPlugin,
                DespawningPlugin,
                EffectsPlugin,
                GameplayLogicPlugin,
                MovementPlugin,
                StartupPlugin,
                UserInterfacePlugin,
            ));
    }
}
