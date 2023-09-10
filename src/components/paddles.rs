use bevy::prelude::{Component, Entity};

/// Makes an entity that can move along a single access inside a goal.
#[derive(Component, Debug)]
pub struct Paddle;

/// Marks a [`Paddle`] entity as being controlled by the keyboard.
#[derive(Component, Debug)]
pub struct KeyboardPlayer;

/// Marks a [`Paddle`] entity as being controlled by AI.
#[derive(Component, Debug)]
pub struct AiPlayer;

/// The [`Ball`] entity targeted by an [`AiPlayer`] [`Paddle`] entity.
#[derive(Clone, Component, Debug)]
#[component(storage = "SparseSet")]
pub struct Target(pub Entity);

/// Marks a ball entity that can collide and score.
#[derive(Component, Debug)]
pub struct Ball;
