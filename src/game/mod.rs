use crate::prelude::*;

mod config;
mod screen;
mod state;
mod system_set;

pub use config::*;
pub use screen::*;
pub use state::*;
pub use system_set::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ScreenPlugin, StatePlugin, SystemSetPlugin));
    }
}
