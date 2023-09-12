use bevy::prelude::{App, Plugin};

pub mod components;
pub mod constants;
pub mod events;
pub mod resources;
pub mod states;
pub mod systems;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            events::EventsPlugin,
            resources::ResourcesPlugin,
            states::ScreensPlugin,
            systems::SystemsPlugin,
        ));
    }
}
