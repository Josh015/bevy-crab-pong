use crate::prelude::*;

/// A component for a corner barrier entity that exists only to deflect `Ball`
/// entities.
#[derive(Component)]
pub struct Barrier;

impl Barrier {
    pub const DIAMETER: f32 = 0.12;
    pub const HEIGHT: f32 = 0.2;
    pub const RADIUS: f32 = 0.5 * Barrier::DIAMETER;
}
