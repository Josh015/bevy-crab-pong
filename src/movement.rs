use crate::prelude::*;
use std::ops::{Add, Sub};

/// Positive or negative force affecting acceleration.
#[derive(Clone, Copy, PartialEq)]
pub enum Force {
    Positive,
    Negative,
}

/// A component that handles all acceleration-based movement for a given entity.
#[derive(Component, Clone, Default)]
pub struct Movement {
    /// Whether the entity has positive/negative acceleration, or is
    /// decelerating if there is none.
    pub force: Option<Force>,
}

/// The normalized direction vector along which the entity will move.
#[derive(Component, Clone, Default)]
pub struct Heading(pub Vec3);

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
    pub heading: Heading,
    pub speed: Speed,
}

#[derive(Bundle, Default)]
pub struct AccelerationBundle {
    #[bundle]
    pub movement2: MovementBundle,
    pub movement: Movement,
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
pub fn acceleration_system(
    time: Res<Time>,
    mut query: Query<(&Movement, &mut Speed, &MaxSpeed, &Acceleration)>,
) {
    for (movement, mut speed, max_speed, acceleration) in &mut query {
        let delta_speed = acceleration.0 * time.delta_seconds();

        speed.0 = if let Some(force) = movement.force {
            // Accelerate
            speed
                .0
                .add(if force == Force::Positive {
                    delta_speed
                } else {
                    -delta_speed
                })
                .clamp(-max_speed.0, max_speed.0)
        } else {
            // Decelerate
            decelerate_speed(speed.0, delta_speed)
        };
    }
}

/// Handles moving entities with [`Heading`] and [`Speed`].
pub fn movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Heading, &Speed)>,
) {
    for (mut transform, heading, speed) in &mut query {
        transform.translation += heading.0 * (speed.0 * time.delta_seconds());
    }
}
