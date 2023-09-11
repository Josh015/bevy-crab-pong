use bevy::prelude::{App, Plugin, Resource};

/// Represents whether the player won or lost the last game.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GameOver {
    Won,
    Lost,
}

/// All the global data for this game.
#[derive(Debug, Default, Resource)]
pub struct GlobalData {
    pub mode_index: usize,
    pub game_over: Option<GameOver>,
    pub is_debugging_enabled: bool,
}

pub struct GlobalDataPlugin;

impl Plugin for GlobalDataPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GlobalData>();
    }
}
