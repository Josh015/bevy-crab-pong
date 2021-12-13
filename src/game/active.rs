use bevy::prelude::*;

/// A component that marks `Ball`, `Paddle`, and `Wall` entities as active for
/// collision, scoring, etc.
///
/// The specific visual implementation of the fade out effect is left up to the
/// compatible component.
#[derive(Component)]
pub struct Active;
