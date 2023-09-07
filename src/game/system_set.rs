use crate::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameSystemSet {
    GameplayLogic,
    Movement,
    Collision,
    Debugging,
    Despawning,
}

fn show_debugging_systems(game_state: Res<GameState>) -> bool {
    game_state.is_debugging_enabled
}

pub struct SystemSetPlugin;

impl Plugin for SystemSetPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            Update,
            GameSystemSet::GameplayLogic
                .before(GameSystemSet::Movement)
                .run_if(in_state(GameScreen::Playing)),
        )
        .configure_set(
            Update,
            GameSystemSet::Movement.run_if(not(in_state(GameScreen::Paused))),
        )
        .configure_set(
            PostUpdate,
            GameSystemSet::Collision
                .after(GameSystemSet::Movement)
                .run_if(not(in_state(GameScreen::Paused))),
        )
        .configure_set(
            Update,
            GameSystemSet::Debugging
                .after(GameSystemSet::Collision)
                .before(GameSystemSet::Despawning)
                .run_if(show_debugging_systems)
                .run_if(not(in_state(GameScreen::StartMenu))),
        )
        .configure_set(
            PostUpdate,
            GameSystemSet::Despawning
                .after(GameSystemSet::Collision)
                .run_if(not(in_state(GameScreen::Paused))),
        );
    }
}
