use bevy::prelude::{Component, Entity};

/// Makes an entity that can move along a single access inside a goal.
#[derive(Clone, Component, Eq, PartialEq, Debug, Hash)]
pub struct Paddle;

/// Marks a [`Paddle`] entity as being controlled by the keyboard.
#[derive(Component)]
pub struct KeyboardPlayer;

/// Marks a [`Paddle`] entity as being controlled by AI.
#[derive(Component)]
pub struct AiPlayer;

/// The [`Ball`] entity targeted by an [`AiPlayer`] [`Paddle`] entity.
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Target(pub Entity);

/// Marks a ball entity that can collide and score.
#[derive(Component)]
pub struct Ball;
