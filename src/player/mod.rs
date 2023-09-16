use bevy::prelude::*;

use crate::{common::movement::MovementSet, state::GameState};

pub mod ai;
pub mod input;

/// Systems that must control [`Crab`](crate::crab::Crab) entities.
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
