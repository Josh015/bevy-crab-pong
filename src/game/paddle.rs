use bevy::{ecs::prelude::*, prelude::*};
use std::ops::{Add, Sub};

use super::{
    fade::{Active, Fade},
    Velocity,
};
use crate::GameConfig;

#[derive(Clone, Component, Eq, PartialEq, Debug, Hash)]
pub enum Paddle {
    Stop,
    Left,
    Right,
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

pub fn acceleration_system(
    config: Res<GameConfig>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity, &Paddle), With<Active>>,
) {
    for (mut transform, mut velocity, paddle) in query.iter_mut() {
        // Accelerate the paddle
        let delta_speed = config.paddle_acceleration() * time.delta_seconds();

        if *paddle == Paddle::Stop {
            let s = velocity.speed.abs().sub(delta_speed).max(0.0);
            velocity.speed = velocity.speed.max(-s).min(s);
        } else {
            velocity.speed = velocity
                .speed
                .add(if *paddle == Paddle::Left {
                    -delta_speed
                } else {
                    delta_speed
                })
                .clamp(-config.paddle_max_speed, config.paddle_max_speed);
        }
    }
}

pub fn bounded_movement_system(
    config: Res<GameConfig>,
    time: Res<Time>,
    mut query: Query<
        (&mut Transform, &mut Velocity),
        (With<Paddle>, With<Active>),
    >,
) {
    for (mut transform, mut velocity) in query.iter_mut() {
        // Limit paddle to open space between barriers
        let extents = 0.5
            * (config.beach_width
                - config.barrier_width
                - config.paddle_scale.0);

        if transform.translation.x > extents {
            transform.translation.x = extents;
            velocity.speed = 0.0;
        } else if transform.translation.x < -extents {
            transform.translation.x = -extents;
            velocity.speed = 0.0;
        }
    }
}

// TODO: Velocity here is more like Fade in that it will just build up speed
// over time until it hits a maximum. For crabs, direction vector will stay the
// same, but speed can be either positive or negative to control which
// way they're moving and allow us to accelerate/decelerate. Can just keep
// velocity

// TODO: To help justify the more general Velocity, maybe balls can have a short
// acceleration after they launch?
