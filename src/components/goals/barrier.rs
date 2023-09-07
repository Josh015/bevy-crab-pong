use crate::prelude::*;

pub const BARRIER_DIAMETER: f32 = 0.12;
pub const BARRIER_RADIUS: f32 = 0.5 * BARRIER_DIAMETER;
pub const BARRIER_HEIGHT: f32 = 0.2;

/// A component for a corner barrier entity that exists only to deflect
/// [`Ball`] entities.
#[derive(Component)]
pub struct Barrier;
