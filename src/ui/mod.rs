mod debug_mode;
mod menu;

pub use debug_mode::*;
pub use menu::*;

use bevy::prelude::*;

pub(super) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DebugModePlugin, MenuPlugin));
    }
}
