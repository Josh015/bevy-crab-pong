use bevy::prelude::*;

use crate::game::state::GameState;

/// Marks a collidable entity.
#[derive(Component, Debug)]
pub struct Collider;

/// For systems that handle collisions.
#[derive(SystemSet, Clone, Hash, Debug, PartialEq, Eq)]
pub struct ColliderSet;

pub(super) struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            PostUpdate,
            ColliderSet.run_if(in_state(GameState::Playing)),
        );
    }
}
