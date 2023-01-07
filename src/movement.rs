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

    /// The current speed of the entity's movement which will largely be
    /// controlled by the component.
    pub speed: f32,

    /// Whether the entity has positive/negative acceleration, or is
    /// decelerating if there is none.
    pub delta: Option<MovementDelta>,
}

/// The maximum speed this entity can reach after accelerating.
#[derive(Component, Clone, Default)]
pub struct MaxSpeed(pub f32);

/// The `max_speed / seconds_to_reach_max_speed`.
#[derive(Component, Clone, Default)]
pub struct Acceleration(pub f32);

#[derive(Bundle, Default)]
pub struct MovementBundle {
    pub movement: Movement,
    pub max_speed: MaxSpeed,
    pub acceleration: Acceleration,
}

impl Movement {
    /// Removes acceleration and immediately sets speed to zero.
    pub fn stop(&mut self) {
        self.delta = None;
        self.speed = 0.0;
    }
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
    mut query: Query<(&mut Transform, &mut Movement, &MaxSpeed, &Acceleration)>,
) {
    for (mut transform, mut movement, max_speed, acceleration) in &mut query {
        let delta_seconds = time.delta_seconds();
        let delta_speed = acceleration.0 * delta_seconds;

        movement.speed = if let Some(delta) = movement.delta {
            // Accelerate
            movement
                .speed
                .add(if delta == MovementDelta::Positive {
                    delta_speed
                } else {
                    -delta_speed
                })
                .clamp(-max_speed.0, max_speed.0)
        } else {
            // Decelerate
            decelerate_speed(movement.speed, delta_speed)
        };

        transform.translation +=
            movement.direction * (movement.speed * delta_seconds);
    }
}
