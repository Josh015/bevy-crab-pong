mod assets;
mod level;
mod state;
mod system_params;

pub use assets::*;
pub use level::*;
pub use state::*;
pub use system_params::*;

use bevy::prelude::*;

pub(super) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AssetsPlugin,
            LevelPlugin,
            StatePlugin,
            SystemParamsPlugin,
        ));
    }
}
