use bevy::{ecs::prelude::*, prelude::*};
use std::ops::{Add, Sub};

#[derive(Component)]
pub struct Velocity {
    pub direction: Vec3,
    pub speed: f32,
    pub max_speed: f32,
    pub acceleration: f32,
    pub delta: Delta,
}

#[derive(Clone, Copy)]
pub enum Delta {
    Decelerating,
    Accelerating(f32),
}

pub fn acceleration_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity)>,
) {
    for (mut transform, mut velocity) in query.iter_mut() {
        let delta_seconds = time.delta_seconds();
        let delta_speed = velocity.acceleration * delta_seconds;

        velocity.speed = match velocity.delta {
            Delta::Decelerating => {
                let s = velocity.speed.abs().sub(delta_speed).max(0.0);
                velocity.speed.max(-s).min(s)
            },
            Delta::Accelerating(direction) => velocity
                .speed
                .add(direction * delta_speed)
                .clamp(-velocity.max_speed, velocity.max_speed),
        };

        transform.translation +=
            velocity.direction * (velocity.speed * delta_seconds);
    }
}
