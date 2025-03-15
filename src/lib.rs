mod assets;
mod components;
mod events;
mod level;
mod state;
mod system_params;
mod system_sets;
mod ui;

pub use assets::*;
pub use components::*;
pub use events::*;
pub use level::*;
pub use state::*;
pub use system_params::*;
pub use system_sets::*;
pub use ui::*;

use bevy::prelude::*;

pub struct LibPlugin;

impl Plugin for LibPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AssetsPlugin,
            ComponentsPlugin,
            EventsPlugin,
            LevelPlugin,
            StatePlugin,
            SystemParamsPlugin,
            SystemSetsPlugin,
            UiPlugin,
        ));
    }
}
