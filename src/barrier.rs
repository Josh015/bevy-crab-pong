use bevy::prelude::*;

pub const BARRIER_DIAMETER: f32 = 0.12;
pub const BARRIER_RADIUS: f32 = 0.5 * BARRIER_DIAMETER;
pub const BARRIER_HEIGHT: f32 = 0.2;

/// Marks an entity as a barrier to deflect all balls away from a corner.
#[derive(Component, Debug)]
pub struct Barrier;
