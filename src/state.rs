use crate::components::goals::Side;
use bevy::prelude::{App, Plugin, Resource};
use std::collections::HashMap;

/// Represents whether the player won or lost the last game.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum GameOver {
    #[default]
    Won,
    Lost,
}

/// All global information for this game.
#[derive(Debug, Resource)]
pub struct GameState {
    pub mode_index: usize,
    pub goals_hit_points: HashMap<Side, u32>,
    pub game_over: Option<GameOver>,
    pub is_debugging_enabled: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            mode_index: 0,
            goals_hit_points: HashMap::with_capacity(4),
            game_over: None,
            is_debugging_enabled: false,
        }
    }
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameState>();
    }
}
