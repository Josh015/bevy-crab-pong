mod all;
mod paused;
mod start_menu;

use bevy::prelude::{App, Plugin, States};

/// Current screen of the game.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum GameScreen {
    #[default]
    StartMenu,
    Playing,
    Paused,
}

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameScreen>().add_plugins((
            all::AllPlugin,
            paused::PausedPlugin,
            start_menu::StartMenuPlugin,
        ));
    }
}
