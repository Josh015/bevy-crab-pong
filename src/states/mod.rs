mod all;
mod loading;
mod paused;
mod start_menu;

use bevy::prelude::{App, Plugin, States};

/// Current screen of the game.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum GameState {
    #[default]
    Loading,
    StartMenu,
    Playing,
    Paused,
}

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().add_plugins((
            all::AllPlugin,
            loading::LoadingPlugin,
            paused::PausedPlugin,
            start_menu::StartMenuPlugin,
        ));
    }
}
