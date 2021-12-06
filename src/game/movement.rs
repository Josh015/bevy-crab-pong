use bevy::{ecs::prelude::*, prelude::*};
use std::ops::{Add, Sub};

#[derive(Component)]
pub struct Movement {
    pub direction: Vec3,
    pub speed: f32,
    pub max_speed: f32,
    pub acceleration: f32,
    pub delta: Option<Delta>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Delta {
    Positive,
    Negative,
}

pub fn acceleration_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Movement)>,
) {
    for (mut transform, mut movement) in query.iter_mut() {
        let delta_seconds = time.delta_seconds();
        let delta_speed = movement.acceleration * delta_seconds;

        movement.speed = if let Some(delta) = movement.delta {
            movement
                .speed
                .add(if delta == Delta::Positive {
                    delta_speed
                } else {
                    -delta_speed
                })
                .clamp(-movement.max_speed, movement.max_speed)
        } else {
            let s = movement.speed.abs().sub(delta_speed).max(0.0);
            movement.speed.max(-s).min(s) // Can't clamp() due to panic when -s == s
        };

        transform.translation +=
            movement.direction * (movement.speed * delta_seconds);
    }
}
