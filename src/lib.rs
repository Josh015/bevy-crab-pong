use bevy::prelude::{App, Plugin};

pub mod components;
pub mod constants;
pub mod events;
pub mod resources;
pub mod states;
pub mod systems;

pub struct BevyCrabPongPlugin;

impl Plugin for BevyCrabPongPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            events::EventsPlugin,
            resources::ResourcesPlugin,
            states::ScreensPlugin,
            systems::SystemsPlugin,
        ));
    }
}
