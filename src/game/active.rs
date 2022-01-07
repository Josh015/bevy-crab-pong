use bevy::prelude::*;

/// A component that marks `Ball`, `Paddle`, and `Wall` entities as active for
/// collision, scoring, etc.
#[derive(Component)]
pub struct Active;
