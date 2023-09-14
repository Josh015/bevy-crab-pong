use bevy::prelude::*;

use crate::{movement::MovementSet, state::GameState};

pub mod ai;
pub mod input;

#[derive(SystemSet, Clone, Hash, Debug, PartialEq, Eq)]
pub struct PlayerSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            Update,
            PlayerSet
                .before(MovementSet)
                .run_if(in_state(GameState::Playing)),
        )
        .add_plugins((ai::AiPlugin, input::InputPlugin));
    }
}
