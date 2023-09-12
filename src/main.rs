#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod components;
mod constants;
mod events;
mod resources;
mod states;
mod systems;

use std::time::Duration;

use bevy::{asset::ChangeWatcher, prelude::*, window::PresentMode};

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
        .add_plugins((
            events::EventsPlugin,
            resources::ResourcesPlugin,
            states::ScreensPlugin,
            systems::SystemsPlugin,
        ))
        .run();
}
