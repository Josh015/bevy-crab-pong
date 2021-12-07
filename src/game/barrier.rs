use bevy::prelude::Component;

/// A component for a corner barrier entity that exists only to deflect `Ball`
/// entities.
#[derive(Component)]
pub struct Barrier;
