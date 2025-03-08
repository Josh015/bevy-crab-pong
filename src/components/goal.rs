use bevy::prelude::*;

/// Marks a goal that can be used as a parent to spawn [`Side`] entities.
#[derive(Component, Debug)]
#[require(Transform, Visibility)]
pub struct Goal;
