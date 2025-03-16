use bevy::prelude::*;
use std::ops::{Add, Sub};

use crate::system_sets::PausableSet;

pub(super) struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (acceleration, deceleration, velocity, stopping_distance)
                .chain()
                .in_set(PausableSet),
        );
    }
}

/// Marks an entity as able to move.
#[derive(Component, Default)]
pub struct Movement;

/// Whether the entity has positive or negative force acting on it.
#[derive(Clone, Component, Copy, Debug, Eq, Hash, PartialEq)]
#[component(storage = "SparseSet")]
pub enum Force {
    Positive,
    Negative,
}

/// The direction in which the entity is moving.
#[derive(Clone, Component, Debug)]
#[require(Speed)]
pub struct Heading(pub Dir3);

impl Default for Heading {
    fn default() -> Self {
        Self(Dir3::NEG_Z)
    }
}

impl From<Vec3> for Heading {
    fn from(value: Vec3) -> Self {
        Heading(Dir3::new_unchecked(value.normalize()))
    }
}

impl Heading {
    pub fn reflect(heading: &Heading, axis: Vec3) -> Self {
        let i = *heading.0;
        let n = axis;
        let r = i - (2.0 * (i.dot(n) * n));

        Heading::from(r)
    }
}

/// The current speed of this entity.
#[derive(Clone, Component, Debug, Default)]
pub struct Speed(pub f32);

/// The maximum speed this entity can reach after accelerating.
#[derive(Clone, Component, Debug, Default)]
pub struct MaxSpeed(pub f32);

/// The `max_speed / seconds_to_reach_max_speed`.
#[derive(Clone, Component, Debug, Default)]
#[require(Heading, MaxSpeed, StoppingDistance)]
pub struct Acceleration(pub f32);

/// Distance from an entity's current position to where it will come to a full
/// stop if it begins decelerating immediately.
#[derive(Clone, Component, Debug, Default)]
pub struct StoppingDistance(pub f32);

fn acceleration(
    time: Res<Time>,
    mut query: Query<
        (&Acceleration, &mut Speed, &Force, &MaxSpeed),
        With<Movement>,
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
        (With<Movement>, Without<Force>),
    >,
) {
    for (acceleration, mut speed) in &mut query {
        let delta_speed = acceleration.0 * time.delta_secs();
        speed.0 = decelerate_speed(speed.0, delta_speed);
    }
}

fn velocity(
    time: Res<Time>,
    mut query: Query<(&Speed, &Heading, &mut Transform), With<Movement>>,
) {
    for (speed, heading, mut transform) in &mut query {
        transform.translation += heading.0 * (speed.0 * time.delta_secs());
    }
}

fn stopping_distance(
    mut query: Query<
        (&mut StoppingDistance, &Acceleration, &Speed),
        With<Movement>,
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
