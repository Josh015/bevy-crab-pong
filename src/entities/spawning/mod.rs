use crate::prelude::*;

mod fade;
mod for_state;

pub use fade::*;
pub use for_state::*;

pub struct SpawningPlugin;

impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((FadePlugin, ForStatePlugin));
    }
}
