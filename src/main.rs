#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod cached_assets;
mod components;
mod constants;
mod events;
mod global_data;
mod screens;
mod serialization;
mod system_sets;

use crate::serialization::*;
use bevy::{
    prelude::*,
    window::{PresentMode, WindowResolution},
};

fn main() {
    let config: Config = load_config_from_ron_file("assets/config/game.ron");

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: config.title.clone(),
                resolution: WindowResolution::new(
                    config.width as f32,
                    config.height as f32,
                ),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Msaa::default())
        .insert_resource(ClearColor(config.clear_color))
        .insert_resource(config)
        .add_plugins((
            cached_assets::CachedAssetsPlugin,
            events::EventsPlugin,
            global_data::GlobalDataPlugin,
            screens::ScreensPlugin,
            system_sets::SystemSetsPlugin,
        ))
        .run();
}
