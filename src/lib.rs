mod assets;
mod components;
mod events;
mod spawners;
mod states;
mod system_params;
mod system_sets;
mod ui;

use bevy::prelude::*;

pub struct LibPlugin;

impl Plugin for LibPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            assets::AssetsPlugin,
            components::ComponentsPlugin,
            events::EventsPlugin,
            spawners::SpawnersPlugin,
            states::StatesPlugin,
            system_params::SystemParamsPlugin,
            system_sets::SystemSetsPlugin,
            ui::UiPlugin,
        ));
    }
}
