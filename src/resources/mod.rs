mod cached_assets;
mod config;
mod global_data;

pub use cached_assets::*;
pub use config::*;
pub use global_data::*;

use bevy::prelude::{App, Plugin};

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            cached_assets::CachedAssetsPlugin,
            global_data::GlobalDataPlugin,
        ));
    }
}
