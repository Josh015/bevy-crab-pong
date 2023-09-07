use crate::prelude::*;

mod arena;
mod balls;
mod control;
mod goals;
mod movement;
mod spawning;

pub use arena::*;
pub use balls::*;
pub use control::*;
pub use goals::*;
pub use movement::*;
pub use spawning::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameSystemSet {
    GameplayLogic,
    Movement,
    Collision,
    Debugging,
    Despawning,
}

fn show_debugging_systems(run_state: Res<RunState>) -> bool {
    run_state.is_debugging_enabled
}

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            Update,
            GameSystemSet::GameplayLogic
                .before(GameSystemSet::Movement)
                .run_if(in_state(GameScreen::Playing)),
        );
        app.configure_set(
            Update,
            GameSystemSet::Movement.run_if(not(in_state(GameScreen::Paused))),
        );
        app.configure_set(
            PostUpdate,
            GameSystemSet::Collision
                .after(GameSystemSet::Movement)
                .run_if(not(in_state(GameScreen::Paused))),
        );
        app.configure_set(
            Update,
            GameSystemSet::Debugging
                .after(GameSystemSet::Collision)
                .before(GameSystemSet::Despawning)
                .run_if(show_debugging_systems)
                .run_if(not(in_state(GameScreen::StartMenu))),
        );
        app.configure_set(
            PostUpdate,
            GameSystemSet::Despawning
                .after(GameSystemSet::Collision)
                .run_if(not(in_state(GameScreen::Paused))),
        );
        app.add_plugins((
            ArenaPlugin,
            BallsPlugin,
            ControlPlugin,
            GoalsPlugin,
            MovementPlugin,
            SpawningPlugin,
        ));
    }
}
