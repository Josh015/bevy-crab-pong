pub mod hud;
pub mod menu;

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((hud::HudPlugin, menu::MenuPlugin));
    }
}
