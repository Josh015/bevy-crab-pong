use crate::prelude::*;

/// Marks an entity as an ocean with an animated texture effect.
#[derive(Component, Default)]
pub struct Ocean {
    pub scroll: f32,
}

/// Marks a [`Camera3d`] entity to sway back and forth in a slow reciprocating
/// motion while looking at the center of the arena.
#[derive(Component)]
pub struct SwayingCamera;
