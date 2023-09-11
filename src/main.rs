#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod cached_assets;
mod components;
mod constants;
mod events;
mod global_data;
mod serialization;
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
            cached_assets::CachedAssetsPlugin,
            events::EventsPlugin,
            global_data::GlobalDataPlugin,
            states::ScreensPlugin,
            systems::SystemsPlugin,
        ))
        .run();
}
