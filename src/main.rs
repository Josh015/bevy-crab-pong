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
use rust_i18n::*;

i18n!("locales", fallback = "en");

fn main() {
    App::new()
        .insert_resource(DefaultOpaqueRendererMethod::deferred())
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: t!("ui.window.title").to_string(),
                        present_mode: PresentMode::AutoVsync,
                        position: WindowPosition::Centered(
                            MonitorSelection::Primary,
                        ),
                        resolution: WindowResolution::new(640.0, 640.0),
                        resize_constraints: WindowResizeConstraints {
                            min_height: 640.0,
                            min_width: 640.0,
                            ..default()
                        },
                        // resizable: false,
                        // enabled_buttons: EnabledButtons {
                        //     maximize: false,
                        //     ..default()
                        // },
                        fit_canvas_to_parent: true,
                        // #[cfg(not(debug_assertions))]
                        // canvas: Some("#pico-td".into()),
                        // window_theme: Some(WindowTheme::Dark),
                        // #[cfg(not(target_os = "windows"))]
                        // visible: false,
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
