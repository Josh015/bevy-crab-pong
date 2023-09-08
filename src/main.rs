#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod cached_assets;
mod components;
mod config;
mod constants;
mod events;
mod file;
mod screens;
mod state;
mod system_sets;

use bevy::{
    prelude::*,
    window::{PresentMode, WindowResolution},
};

fn main() {
    let game_config: config::GameConfig =
        file::load_config_from_file("assets/config/game.ron");

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: game_config.title.clone(),
                resolution: WindowResolution::new(
                    game_config.width as f32,
                    game_config.height as f32,
                ),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Msaa::default())
        .insert_resource(ClearColor(game_config.clear_color))
        .insert_resource(game_config)
        .add_plugins((
            cached_assets::CachedAssetsPlugin,
            events::EventsPlugin,
            screens::ScreensPlugin,
            state::StatePlugin,
            system_sets::SystemSetsPlugin,
        ))
        .run();
}
