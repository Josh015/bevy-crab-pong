#![allow(clippy::too_many_arguments, clippy::type_complexity)]

pub mod common;
pub mod game;
pub mod level;
pub mod spawners;
pub mod ui;
pub mod util;

use bevy::{
    prelude::*,
    window::{PresentMode, WindowResolution},
};

fn main() {
    App::new()
        .add_plugins(
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
        )
        .insert_resource(Msaa::default())
        .insert_resource(ClearColor(Color::srgba(0.7, 0.9, 1.0, 1.0)))
        .add_plugins((
            common::CommonPlugin,
            game::GamePlugin,
            level::LevelPlugin,
            spawners::SpawnersPlugin,
            ui::UiPlugin,
        ))
        .run();
}
