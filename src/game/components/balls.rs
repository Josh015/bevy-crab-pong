use crate::prelude::*;

/// Marks an entity that can be collided with by a [`Ball`] entity.
#[derive(Component)]
pub struct Collider;

/// Marks a ball entity that must deflect upon collision with a [`Collider`]
/// and score when entering a [`Goal`].
#[derive(Component)]
pub struct Ball;
