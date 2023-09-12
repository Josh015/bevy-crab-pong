pub mod cached_assets;
pub mod config;
pub mod global_data;

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
