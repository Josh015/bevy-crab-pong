#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::{asset::ChangeWatcher, prelude::*, window::PresentMode};
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
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
        .add_plugins(bevy_crab_pong::BevyCrabPongPlugin)
        .run();
}
