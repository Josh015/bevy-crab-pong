use bevy::prelude::*;

use crate::state::AppState;

/// Marks a collidable entity.
#[derive(Component, Debug)]
pub struct Collider;

#[derive(SystemSet, Clone, Hash, Debug, PartialEq, Eq)]
pub struct ColliderSet;

pub struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            PostUpdate,
            ColliderSet.run_if(in_state(AppState::Playing)),
        );
    }
}
