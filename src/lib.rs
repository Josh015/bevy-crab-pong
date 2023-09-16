pub mod common;
pub mod debug_mode;
pub mod game;
pub mod level;
pub mod object;
pub mod player;
pub mod ui;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            common::CommonPlugin,
            debug_mode::DebugModePlugin,
            game::GamePlugin,
            level::LevelPlugin,
            object::ObjectPlugin,
            player::PlayerPlugin,
            ui::UiPlugin,
        ));
    }
}
