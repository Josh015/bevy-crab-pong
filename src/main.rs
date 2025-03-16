#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod assets;
mod components;
mod events;
mod spawners;
mod states;
mod system_params;
mod system_sets;
mod ui;

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
        .add_plugins((
            assets::AssetsPlugin,
            components::ComponentsPlugin,
            events::EventsPlugin,
            spawners::SpawnersPlugin,
            states::StatesPlugin,
            system_params::SystemParamsPlugin,
            system_sets::SystemSetsPlugin,
            ui::UiPlugin,
        ))
        .insert_resource(ClearColor(Color::srgba(0.7, 0.9, 1.0, 1.0)))
        .run();
}
