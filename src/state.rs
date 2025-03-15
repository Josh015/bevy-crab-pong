use bevy::prelude::*;
use strum::EnumIter;

pub(super) struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) { app.init_state::<GameState>(); }
}

// All the app's possible states.
#[derive(
    Clone, Copy, Debug, Default, EnumIter, Eq, Hash, PartialEq, States,
)]
pub enum GameState {
    #[default]
    Loading,
    StartMenu,
    Playing,
    Paused,
}
