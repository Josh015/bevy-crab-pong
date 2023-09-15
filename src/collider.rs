use bevy::prelude::*;

use crate::state::GameState;

/// Marks a collidable entity.
#[derive(Component, Debug)]
pub struct Collider;

/// For systems that handle collisions that modify a [`Ball`](crate::ball::Ball) [`Heading`](crate::movement::Heading).
#[derive(SystemSet, Clone, Hash, Debug, PartialEq, Eq)]
pub struct ColliderSet;

pub struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            PostUpdate,
            ColliderSet.run_if(in_state(GameState::Playing)),
        );
    }
}

// TODO: Add debug visualizations for bounding shapes?
