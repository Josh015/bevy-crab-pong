use bevy::prelude::{Component, Entity};

/// Marks a [`Text`] entity to display the HP for an associated [`Paddle`].
#[derive(Component)]
pub struct HitPointsUi;

/// Assigns a [`Paddle`] to a team to determine the winner of each round.
#[derive(Clone, Component, Copy, Default, PartialEq, Debug)]
pub enum Team {
    #[default]
    Enemies,
    Allies,
}

/// Makes an entity that can move along a single access inside a goal.
#[derive(Clone, Component, Eq, PartialEq, Debug, Hash)]
pub struct Paddle;

/// Marks a [`Paddle`] entity as being controlled by the keyboard.
#[derive(Component)]
pub struct KeyboardInput;

/// Marks a [`Paddle`] entity as being controlled by AI.
#[derive(Component)]
pub struct AiInput;

/// The [`Ball`] entity targeted by an [`AiInput`] [`Paddle`] entity.
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Target(pub Entity);
