mod all;
mod loading;
mod paused;
mod start_menu;

use bevy::prelude::{App, Plugin};

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            all::AllPlugin,
            loading::LoadingPlugin,
            paused::PausedPlugin,
            start_menu::StartMenuPlugin,
        ));
    }
}
