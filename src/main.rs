#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod assets;
mod components;
mod spawners;
mod states;
mod system_params;
mod system_sets;
mod ui;

use bevy::{
    core_pipeline::experimental::taa::TemporalAntiAliasPlugin,
    pbr::DefaultOpaqueRendererMethod,
    prelude::*,
    window::{PresentMode, WindowResolution},
};

fn main() {
    App::new()
        .insert_resource(DefaultOpaqueRendererMethod::deferred())
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
            TemporalAntiAliasPlugin,
            // ScreenSpaceAmbientOcclusionPlugin,
            // ScreenSpaceReflectionsPlugin,
        ))
        .add_plugins((
            assets::AssetsPlugin,
            components::ComponentsPlugin,
            spawners::SpawnersPlugin,
            states::StatesPlugin,
            system_params::SystemParamsPlugin,
            system_sets::SystemSetsPlugin,
            ui::UiPlugin,
        ))
        .insert_resource(ClearColor(Color::srgba(0.7, 0.9, 1.0, 1.0)))
        .run();
}
