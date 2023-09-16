#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::{
    asset::ChangeWatcher,
    prelude::*,
    window::{PresentMode, WindowResolution},
};
use std::time::Duration;

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
                    watch_for_changes: ChangeWatcher::with_delay(
                        Duration::from_secs(1),
                    ),
                    ..Default::default()
                }),
        )
        .insert_resource(Msaa::default())
        .insert_resource(ClearColor(Color::rgba(0.7, 0.9, 1.0, 1.0)))
        .add_plugins(bevy_crab_pong::BevyCrabPongPlugin)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
