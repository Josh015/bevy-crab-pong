use crate::prelude::*;

/// Marks a component that can collide, score, etc.
#[derive(Component)]
pub struct Collider;

/// A component for marking a [`Paddle`] entity as being controller by player
/// input.
#[derive(Component)]
pub struct Player; // TODO: Eliminate this component entirely?

/// A component that works in tandem with [`Paddle`] to make AI-driven
/// opponents.
#[derive(Component)]
pub struct Enemy;

/// A component that makes a paddle that can deflect [`Ball`] entities and
/// moves left->right and vice versa along a single axis when [`Collider`].
#[derive(Clone, Component, Eq, PartialEq, Debug, Hash)]
pub struct Paddle;

/// A component for a corner barrier entity that exists only to deflect
/// [`Ball`] entities.
#[derive(Component)]
pub struct Barrier;

/// A component for a ball entity that must have inertia and be able to deflect
/// upon collision when [`Collider`].
#[derive(Component)]
pub struct Ball;

/// A component that makes an entity a wall in a [`Goal`] that can deflect
/// [`Ball`] entities away from the entire goal when [`Collider`].
#[derive(Component)]
pub struct Wall;
