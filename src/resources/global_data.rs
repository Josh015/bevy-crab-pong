use bevy::prelude::{App, Plugin, Resource};

/// The currently-selected game mode.
#[derive(Debug, Default, Resource)]
pub struct SelectedGameMode(pub usize);

pub struct GlobalDataPlugin;

impl Plugin for GlobalDataPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedGameMode>();
    }
}
