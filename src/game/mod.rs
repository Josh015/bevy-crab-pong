pub mod assets;
pub mod level;
pub mod state;
pub mod system_params;

use bevy::prelude::*;

pub(super) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            assets::AssetsPlugin,
            level::LevelPlugin,
            state::StatePlugin,
            system_params::SystemParamsPlugin,
        ));
    }
}
