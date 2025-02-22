use bevy::{ecs::system::SystemParam, prelude::*};
use std::ops::Add;

use super::assets::{GameAssets, GameMode};

#[derive(Debug, Default, Resource)]
struct SelectedGameMode(usize);

/// Allows systems to query and set the current game mode.
#[derive(SystemParam)]
pub struct GameModes<'w> {
    game_assets: Res<'w, GameAssets>,
    game_modes: Res<'w, Assets<GameMode>>,
    selected: ResMut<'w, SelectedGameMode>,
}

impl<'w> GameModes<'w> {
    /// Gets the current game mode.
    pub fn current(&self) -> &GameMode {
        self.game_modes
            .get(&self.game_assets.game_modes[self.selected.0])
            .unwrap()
    }

    /// Switch to the previous game mode.
    pub fn previous(&mut self) {
        self.selected.0 = self.selected.0.saturating_sub(1);
    }

    /// Switch to the next game mode.
    pub fn next(&mut self) {
        self.selected.0 = self
            .selected
            .0
            .add(1)
            .min(self.game_assets.game_modes.len() - 1);
    }
}

pub(super) struct ModesPlugin;

impl Plugin for ModesPlugin {
    fn build(&self, app: &mut App) { app.init_resource::<SelectedGameMode>(); }
}
