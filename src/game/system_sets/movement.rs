#![allow(clippy::type_complexity)]

use crate::prelude::*;
use std::ops::{Add, Sub};

/// Calculate a new reduced speed value based on delta speed and clamping
/// to zero.
fn decelerate_speed(speed: f32, delta_speed: f32) -> f32 {
    let s = speed.abs().sub(delta_speed).max(0.0);
    speed.max(-s).min(s) // clamp() panics when min == max.
}

/// Handles acceleration over time for entities with [`Force`].
fn acceleration(
    time: Res<Time>,
    mut query: Query<(&mut Speed, &Acceleration, &Force, &MaxSpeed)>,
) {
    for (mut speed, acceleration, force, max_speed) in &mut query {
        let delta_speed = acceleration.0 * time.delta_seconds();

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

/// Handles deceleration over time for entities without [`Force`].
fn deceleration(
    time: Res<Time>,
    mut query: Query<(&mut Speed, &Acceleration), Without<Force>>,
) {
    for (mut speed, acceleration) in &mut query {
        let delta_speed = acceleration.0 * time.delta_seconds();
        speed.0 = decelerate_speed(speed.0, delta_speed);
    }
}

/// Handles moving entities with a [`Heading`] and [`Speed`].
fn velocity(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Heading, &Speed)>,
) {
    for (mut transform, heading, speed) in &mut query {
        transform.translation += heading.0 * (speed.0 * time.delta_seconds());
    }
}

/// Calculates the stopping distance for an entity.
fn stopping_distance(
    mut query: Query<(&mut StoppingDistance, &Acceleration, &Speed)>,
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

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (acceleration, deceleration, velocity, stopping_distance)
                .chain()
                .in_set(GameSystemSet::Movement),
        );
    }
}