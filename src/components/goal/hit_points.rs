use bevy::prelude::*;

use super::Goal;

/// How many balls a [`Goal`] can take before it's eliminated.
#[derive(Component, Debug, Default)]
#[require(Goal)]
pub struct HitPoints(pub u8);
