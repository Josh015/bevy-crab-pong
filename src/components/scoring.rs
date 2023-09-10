use bevy::prelude::Component;

/// Marks a [`Text`] entity to display the HP for an associated [`Paddle`].
#[derive(Component)]
pub struct HitPointsUi;

// An entity's HP for checking win conditions.
#[derive(Component)]
pub struct HitPoints(pub u8);

// An entity's team for checking win conditions.
#[derive(Component)]
pub struct Team(pub u8);
