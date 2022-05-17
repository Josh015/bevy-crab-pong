use crate::prelude::*;
use std::ops::{Add, Sub};

/// Represents whether the movement has positive or negative acceleration.
#[derive(Clone, Copy, PartialEq)]
pub enum Delta {
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

    /// The maximum speed this entity can reach after accelerating.
    pub max_speed: f32,

    /// The `max_speed / seconds_to_reach_max_speed`.
    pub acceleration: f32,

    /// Whether the entity has positive/negative acceleration, or is
    /// decelerating if there is none.
    pub delta: Option<Delta>,
}

impl Movement {
    /// Removes acceleration and immediately sets speed to zero.
    pub fn stop(&mut self) {
        self.delta = None;
        self.speed = 0.0;
    }
}

/// Handles calculating the actual acceleration/deceleration over time for a
/// `Movement` entity.
pub fn movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Movement)>,
) {
    for (mut transform, mut movement) in query.iter_mut() {
        let delta_seconds = time.delta_seconds();
        let delta_speed = movement.acceleration * delta_seconds;

        movement.speed = if let Some(delta) = movement.delta {
            // Accelerate
            movement
                .speed
                .add(if delta == Delta::Positive {
                    delta_speed
                } else {
                    -delta_speed
                })
                .clamp(-movement.max_speed, movement.max_speed)
        } else {
            // Decelerate
            let s = movement.speed.abs().sub(delta_speed).max(0.0);
            movement.speed.max(-s).min(s) // clamp() panics when min == max.
        };

        transform.translation +=
            movement.direction * (movement.speed * delta_seconds);
    }
}
