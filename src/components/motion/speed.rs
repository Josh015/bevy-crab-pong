use bevy::prelude::*;

use crate::system_sets::StopWhenPausedSet;

use super::{Direction, Motion};

pub(super) struct SpeedPlugin;

impl Plugin for SpeedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, velocity.in_set(StopWhenPausedSet));
    }
}

/// The current speed of this entity.
#[derive(Clone, Component, Debug, Default)]
#[require(Direction)]
pub struct Speed(pub f32);

fn velocity(
    time: Res<Time>,
    mut query: Query<(&Speed, &Direction, &mut Transform), With<Motion>>,
) {
    for (speed, direction, mut transform) in &mut query {
        transform.translation += direction.0 * (speed.0 * time.delta_secs());
    }
}
