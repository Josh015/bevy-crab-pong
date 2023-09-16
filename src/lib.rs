pub mod common;
pub mod game;
pub mod level;
pub mod object;
pub mod player;
pub mod ui;

use bevy::prelude::*;

pub struct BevyCrabPongPlugin;

impl Plugin for BevyCrabPongPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            common::CommonPlugin,
            game::GamePlugin,
            level::LevelPlugin,
            object::ObjectPlugin,
            player::PlayerPlugin,
            ui::UiPlugin,
        ));
    }
}
