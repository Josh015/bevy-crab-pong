use crate::prelude::*;
use std::ops::{Add, Sub};

/// Represents whether the movement has positive or negative acceleration.
#[derive(Clone, Copy, PartialEq)]
pub enum MovementDelta {
    Positive,
    Negative,
}

/// A component that handles all acceleration-based movement for a given entity.
#[derive(Component, Clone, Default)]
pub struct Movement {
    /// The normalized direction vector along which the entity will move.
    pub direction: Vec3,

    /// Whether the entity has positive/negative acceleration, or is
    /// decelerating if there is none.
    pub delta: Option<MovementDelta>,
}

/// The current speed of this entity.
#[derive(Component, Clone, Default)]
pub struct Speed(pub f32);

/// The maximum speed this entity can reach after accelerating.
#[derive(Component, Clone, Default)]
pub struct MaxSpeed(pub f32);

/// The `max_speed / seconds_to_reach_max_speed`.
#[derive(Component, Clone, Default)]
pub struct Acceleration(pub f32);

#[derive(Bundle, Default)]
pub struct MovementBundle {
    pub movement: Movement,
    pub speed: Speed,
    pub max_speed: MaxSpeed,
    pub acceleration: Acceleration,
}

/// Calculate a new reduced speed value based on delta speed and clamping
/// to zero.
pub fn decelerate_speed(speed: f32, delta_speed: f32) -> f32 {
    let s = speed.abs().sub(delta_speed).max(0.0);
    speed.max(-s).min(s) // clamp() panics when min == max.
}

/// Handles calculating the actual acceleration/deceleration over time for a
/// [`Movement`] entity.
pub fn movement_system(
    time: Res<Time>,
    mut query: Query<(
        &mut Transform,
        &Movement,
        &mut Speed,
        &MaxSpeed,
        &Acceleration,
    )>,
) {
    for (mut transform, movement, mut speed, max_speed, acceleration) in
        &mut query
    {
        let delta_seconds = time.delta_seconds();
        let delta_speed = acceleration.0 * delta_seconds;

        speed.0 = if let Some(delta) = movement.delta {
            // Accelerate
            speed
                .0
                .add(if delta == MovementDelta::Positive {
                    delta_speed
                } else {
                    -delta_speed
                })
                .clamp(-max_speed.0, max_speed.0)
        } else {
            // Decelerate
            decelerate_speed(speed.0, delta_speed)
        };

        transform.translation += movement.direction * (speed.0 * delta_seconds);
    }
}
