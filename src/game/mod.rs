use crate::prelude::*;

mod cached_assets;
mod components;
mod config;
mod constants;
mod events;
mod screens;
mod state;
mod system_sets;

pub use cached_assets::*;
pub use components::*;
pub use config::*;
pub use constants::*;
pub use events::*;
pub use screens::*;
pub use state::*;
pub use system_sets::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CachedAssetsPlugin,
            EventsPlugin,
            ScreensPlugin,
            StatePlugin,
            SystemSetsPlugin,
        ));
    }
}
