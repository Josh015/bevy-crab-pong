use crate::prelude::*;

mod all;
mod paused;
mod start_menu;

pub use all::*;
pub use paused::*;
pub use start_menu::*;

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
            AllPlugin,
            PausedPlugin,
            StartMenuPlugin,
        ));
    }
}
