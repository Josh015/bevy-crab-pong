use bevy::{ecs::prelude::*, prelude::*};
use std::ops::{Add, Sub};

use super::fade::{Active, Fade};
use crate::GameConfig;

#[derive(Clone, PartialEq, Debug)]
pub enum Movement {
    Decelerating,
    Accelerating(f32),
}

impl Default for Movement {
    fn default() -> Self { Self::Decelerating }
}

#[derive(Component, Default)]
pub struct Paddle {
    pub movement: Movement,
    pub speed: f32,
}

pub fn step_fade_animation_system(
    config: Res<GameConfig>,
    mut query: Query<(&mut Transform, &Fade), With<Paddle>>,
) {
    // Grow/Shrink paddles to show/hide them
    for (mut transform, fade) in query.iter_mut() {
        transform.scale = config.paddle_scale.into();
        transform.scale *= fade.opacity();
    }
}

pub fn movement_system(
    config: Res<GameConfig>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Paddle), With<Active>>,
) {
    for (mut transform, mut paddle) in query.iter_mut() {
        // Accelerate the paddle
        let delta_speed = config.paddle_acceleration() * time.delta_seconds();

        match paddle.movement {
            Movement::Decelerating => {
                let s = paddle.speed.abs().sub(delta_speed).max(0.0);
                paddle.speed = paddle.speed.max(-s).min(s);
            },
            Movement::Accelerating(direction) => {
                paddle.speed = paddle
                    .speed
                    .add(direction * delta_speed)
                    .clamp(-config.paddle_max_speed, config.paddle_max_speed);
            },
        }

        // Limit paddle to open space between barriers
        let mut position = transform.translation.x + paddle.speed;
        let extents = 0.5
            * (config.beach_width
                - config.barrier_width
                - config.paddle_scale.0);

        if position >= extents {
            position = extents;
            paddle.speed = 0.0;
        } else if position <= -extents {
            position = -extents;
            paddle.speed = 0.0;
        }

        // Move the paddle
        transform.translation.x = position;
    }
}

// TODO: Idea, have crab movement based on modified Velocity. Velocity here is
// more like Fade in that it will just build up speed over time until it hits a
// maximum. Crab movement system is then based on Changed<Movement>, only
// manipulating the Velocity when it changes. For crabs, direction vector will
// stay the same, but speed can be either positive or negative to control which
// way they're moving and allow us to accelerate/decelerate. Can just keep
// velocity

// TODO: To help justify the more general Velocity, maybe balls can have a short
// acceleration after they launch?

// TODO: Instead of constantly removing Velocity, do major setup at entity
// creation time, then at runtime only change direction, current_speed, then a
// value for if they're accelerating positive/negative or decelerating?
