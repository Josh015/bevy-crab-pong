use bevy::prelude::*;

/// Marks a [`Text`] entity to display the HP for the associated [`HitPoints`].
#[derive(Component, Debug)]
pub struct HitPointsUi;

// An entity's HP for checking win conditions.
#[derive(Clone, Component, Debug)]
pub struct HitPoints(pub u8);

// An entity's team for checking win conditions.
#[derive(Clone, Component, Debug)]
pub struct Team(pub u8);
