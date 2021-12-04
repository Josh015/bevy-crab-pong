use bevy::{ecs::prelude::*, prelude::*};
use std::ops::{Add, Sub};

use super::fade::{Active, Fade};
use crate::GameConfig;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum Movement {
    Stopped,
    Left,
    Right,
}

impl Default for Movement {
    fn default() -> Self { Self::Stopped }
}

#[derive(Component, Default)]
pub struct Paddle {
    pub movement: Movement,
    pub speed: f32,
}

pub(super) fn step_fade_animation_system(
    config: Res<GameConfig>,
    mut query: Query<(&mut Transform, &Fade), With<Paddle>>,
) {
    // Grow/Shrink paddles to show/hide them
    for (mut transform, fade) in query.iter_mut() {
        transform.scale = config.paddle_scale.into();
        transform.scale *= fade.opacity();
    }
}

pub(super) fn movement_system(
    config: Res<GameConfig>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Paddle), With<Active>>,
) {
    for (mut transform, mut paddle) in query.iter_mut() {
        // Accelerate the paddle
        let delta_speed = config.paddle_acceleration() * time.delta_seconds();

        if paddle.movement == Movement::Stopped {
            let s = paddle.speed.abs().sub(delta_speed).max(0.0);
            paddle.speed = paddle.speed.max(-s).min(s);
        } else {
            paddle.speed = paddle
                .speed
                .add(if paddle.movement == Movement::Left {
                    -delta_speed
                } else {
                    delta_speed
                })
                .clamp(-config.paddle_max_speed, config.paddle_max_speed);
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
