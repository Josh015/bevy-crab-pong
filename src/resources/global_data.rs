use bevy::prelude::{App, Plugin, Resource};

/// The currently-selected game mode.
#[derive(Debug, Default, Resource)]
pub struct SelectedGameMode(pub usize);

/// Indicates which team won the last round.
#[derive(Debug, Default, Resource)]
pub struct WinningTeam(pub usize);

/// Toggles displaying debugging gizmos.
#[derive(Debug, Default, Resource)]
pub struct IsDebuggingMode(pub bool);

pub struct GlobalDataPlugin;

impl Plugin for GlobalDataPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedGameMode>()
            .init_resource::<IsDebuggingMode>();
    }
}
