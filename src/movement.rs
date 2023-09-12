use bevy::prelude::*;
use std::ops::{Add, Sub};

use crate::{spawning::Spawning, state::AppState};

/// Whether the entity has positive or negative force acting on it.
#[derive(Clone, Component, Copy, Debug, Eq, Hash, PartialEq)]
#[component(storage = "SparseSet")]
pub enum Force {
    Positive,
    Negative,
}

/// The normalized direction vector along which the entity will move.
#[derive(Clone, Component, Debug, Default)]
pub struct Heading(pub Vec3);

/// The current speed of this entity.
#[derive(Clone, Component, Debug, Default)]
pub struct Speed(pub f32);

/// The maximum speed this entity can reach after accelerating.
#[derive(Clone, Component, Debug, Default)]
pub struct MaxSpeed(pub f32);

/// The `max_speed / seconds_to_reach_max_speed`.
#[derive(Clone, Component, Debug, Default)]
pub struct Acceleration(pub f32);

/// Distance from an entity's current position to where it will come to a full
/// stop if it begins decelerating immediately.
#[derive(Clone, Component, Debug, Default)]
pub struct StoppingDistance(pub f32);

/// Marks an entity as moving with a fixed speed and direction.
#[derive(Bundle, Clone, Debug, Default)]
pub struct VelocityBundle {
    pub heading: Heading,
    pub speed: Speed,
}

/// Marks an entity that accelerates and decelerates when [`Force`] is applied.
#[derive(Bundle, Clone, Debug, Default)]
pub struct AccelerationBundle {
    pub velocity: VelocityBundle,
    pub max_speed: MaxSpeed,
    pub acceleration: Acceleration,
    pub stopping_distance: StoppingDistance,
}

#[derive(SystemSet, Clone, Hash, Debug, PartialEq, Eq)]
pub struct MovementSet;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            Update,
            MovementSet
                .run_if(not(in_state(AppState::Loading)))
                .run_if(not(in_state(AppState::Paused))),
        )
        .add_systems(
            Update,
            (acceleration, deceleration, velocity, stopping_distance)
                .chain()
                .in_set(MovementSet),
        );
    }
}

fn decelerate_speed(speed: f32, delta_speed: f32) -> f32 {
    let s = speed.abs().sub(delta_speed).max(0.0);
    speed.max(-s).min(s) // clamp() panics when min == max.
}

fn acceleration(
    time: Res<Time>,
    mut query: Query<
        (&mut Speed, &Acceleration, &Force, &MaxSpeed),
        Without<Spawning>,
    >,
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

fn deceleration(
    time: Res<Time>,
    mut query: Query<
        (&mut Speed, &Acceleration),
        (Without<Force>, Without<Spawning>),
    >,
) {
    for (mut speed, acceleration) in &mut query {
        let delta_speed = acceleration.0 * time.delta_seconds();
        speed.0 = decelerate_speed(speed.0, delta_speed);
    }
}

fn velocity(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Heading, &Speed), Without<Spawning>>,
) {
    for (mut transform, heading, speed) in &mut query {
        transform.translation += heading.0 * (speed.0 * time.delta_seconds());
    }
}

fn stopping_distance(
    mut query: Query<
        (&mut StoppingDistance, &Acceleration, &Speed),
        Without<Spawning>,
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
