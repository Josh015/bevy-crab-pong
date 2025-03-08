#![allow(clippy::too_many_arguments, clippy::type_complexity)]

pub mod components;
pub mod game;
pub mod ui;
pub mod util;

use bevy::{
    // core_pipeline::experimental::taa::TemporalAntiAliasPlugin,
    prelude::*,
    window::{PresentMode, WindowResolution},
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Crab Pong".to_owned(),
                        resolution: WindowResolution::new(800.0, 800.0),
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    ..Default::default()
                }),
            // TemporalAntiAliasPlugin,
        ))
        .insert_resource(ClearColor(Color::srgba(0.7, 0.9, 1.0, 1.0)))
        .add_plugins((
            components::ComponentsPlugin,
            game::GamePlugin,
            ui::UiPlugin,
        ))
        .run();
}
