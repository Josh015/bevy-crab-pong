use bevy::prelude::{App, Plugin, Resource};

/// All the global data for this game.
#[derive(Debug, Default, Resource)]
pub struct GlobalData {
    pub mode_index: usize,
    pub winning_team: Option<usize>,
    pub is_debugging_enabled: bool,
}

pub struct GlobalDataPlugin;

impl Plugin for GlobalDataPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GlobalData>();
    }
}
