use crate::prelude::*;

mod ai;
mod keyboard;

pub use ai::*;
pub use keyboard::*;

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AiPlugin, KeyboardPlugin));
    }
}
