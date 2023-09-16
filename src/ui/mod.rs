pub mod debug_mode;
pub mod hud;
pub mod menu;

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            debug_mode::DebugModePlugin,
            hud::HudPlugin,
            menu::MenuPlugin,
        ));
    }
}
