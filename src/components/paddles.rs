use bevy::prelude::{Component, Entity};

use crate::serialization::Team;

/// Marks a [`Text`] entity to display the HP for an associated [`Paddle`].
#[derive(Component)]
pub struct HitPointsUi;

/// Makes an entity that can move along a single access inside a goal.
#[derive(Clone, Component, Eq, PartialEq, Debug, Hash)]
pub struct Paddle {
    pub hit_points: u32,
    pub team: Team,
}

/// Marks a [`Paddle`] entity as being controlled by the keyboard.
#[derive(Component)]
pub struct KeyboardPlayer;

/// Marks a [`Paddle`] entity as being controlled by AI.
#[derive(Component)]
pub struct AiPlayer;

/// The [`Ball`] entity targeted by an [`AiInput`] [`Paddle`] entity.
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Target(pub Entity);

/// Marks a ball entity that can collide and score.
#[derive(Component)]
pub struct Ball;
