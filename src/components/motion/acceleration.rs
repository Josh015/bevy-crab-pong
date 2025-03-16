use bevy::prelude::*;
use std::ops::{Add, Sub};

use crate::system_sets::StopWhenPausedSet;

use super::{Direction, Motion, Speed};

pub(super) struct AccelerationPlugin;

impl Plugin for AccelerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (acceleration, deceleration, stopping_distance)
                .chain()
                .in_set(StopWhenPausedSet),
        );
    }
}

/// The `max_speed / seconds_to_reach_max_speed`.
#[derive(Clone, Component, Debug, Default)]
#[require(Direction, MaxSpeed, StoppingDistance)]
pub struct Acceleration(pub f32);

/// Whether the entity has positive or negative force acting on it.
#[derive(Clone, Component, Copy, Debug, Eq, Hash, PartialEq)]
#[component(storage = "SparseSet")]
pub enum Force {
    Positive,
    Negative,
}

/// The maximum speed this entity can reach after accelerating.
#[derive(Clone, Component, Debug, Default)]
#[require(Speed)]
pub struct MaxSpeed(pub f32);

/// Distance from an entity's current position to where it will come to a full
/// stop if it begins decelerating immediately.
#[derive(Clone, Component, Debug, Default)]
pub struct StoppingDistance(pub f32);

fn acceleration(
    time: Res<Time>,
    mut query: Query<
        (&Acceleration, &mut Speed, &Force, &MaxSpeed),
        With<Motion>,
    >,
) {
    for (acceleration, mut speed, force, max_speed) in &mut query {
        let delta_speed = acceleration.0 * time.delta_secs();

        speed.0 = speed
            .0
            .add(if *force == Force::Positive {
                delta_speed
            } else {
                -delta_speed
            })
            .clamp(-max_speed.0, max_speed.0);
    }
}

fn deceleration(
    time: Res<Time>,
    mut query: Query<
        (&Acceleration, &mut Speed),
        (With<Motion>, Without<Force>),
    >,
) {
    for (acceleration, mut speed) in &mut query {
        let delta_speed = acceleration.0 * time.delta_secs();
        speed.0 = decelerate_speed(speed.0, delta_speed);
    }
}

fn stopping_distance(
    mut query: Query<
        (&mut StoppingDistance, &Acceleration, &Speed),
        With<Motion>,
    >,
) {
    for (mut stopping_distance, acceleration, speed) in &mut query {
        const DELTA_SECONDS: f32 = 0.01;
        let delta_speed = acceleration.0 * DELTA_SECONDS;
        let mut current_speed = speed.0;

        stopping_distance.0 = 0f32;

        while current_speed.abs() > 0.0 {
            stopping_distance.0 += current_speed * DELTA_SECONDS;
            current_speed = decelerate_speed(current_speed, delta_speed);
        }
    }
}

fn decelerate_speed(speed: f32, delta_speed: f32) -> f32 {
    speed.abs().sub(delta_speed).max(0.0).copysign(speed)
}
