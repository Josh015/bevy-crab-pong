use super::*;
use crate::GameConfig;
use bevy::{ecs::prelude::*, prelude::*};
use std::ops::{Add, Sub};

#[derive(Clone, Component, Eq, PartialEq, Debug, Hash)]
pub enum Paddle {
    Stop,
    Left,
    Right,
}

pub fn add_velocity_system(
    mut commands: Commands,
    config: Res<GameConfig>,
    query: Query<Entity, (With<Paddle>, Added<Active>, Without<Velocity>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(Velocity {
            direction: Vec3::X,
            speed: 0.0,
            max_speed: config.paddle_max_speed,
            acceleration: config.paddle_max_speed
                / config.paddle_seconds_to_max_speed,
            delta: Delta::Decelerating,
        });
    }
}

pub fn remove_velocity_system(
    mut commands: Commands,
    query: Query<Entity, (With<Paddle>, Without<Active>, With<Velocity>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).remove::<Velocity>();
    }
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
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity, &Paddle), With<Active>>,
) {
    for (mut transform, mut velocity, paddle) in query.iter_mut() {
        velocity.delta = match *paddle {
            Paddle::Stop => Delta::Decelerating,
            Paddle::Left => Delta::Accelerating(-1.0),
            Paddle::Right => Delta::Accelerating(1.0),
        };
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
        let extents =
            0.5 * (ARENA_WIDTH - config.barrier_width - config.paddle_scale.0);

        if transform.translation.x > extents {
            transform.translation.x = extents;
            velocity.speed = 0.0;
        } else if transform.translation.x < -extents {
            transform.translation.x = -extents;
            velocity.speed = 0.0;
        }
    }
}
