use super::*;
use crate::GameConfig;
use bevy::{ecs::prelude::*, prelude::*};
use std::ops::{Add, Sub};

#[derive(Clone, Component, Eq, PartialEq, Debug, Hash)]
pub struct Paddle;

pub fn add_movement_system(
    mut commands: Commands,
    config: Res<GameConfig>,
    query: Query<Entity, (With<Paddle>, Added<Active>, Without<Movement>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(Movement {
            direction: Vec3::X,
            speed: 0.0,
            max_speed: config.paddle_max_speed,
            acceleration: config.paddle_max_speed
                / config.paddle_seconds_to_max_speed,
            delta: None,
        });
    }
}

pub fn remove_movement_system(
    mut commands: Commands,
    query: Query<Entity, (With<Paddle>, Without<Active>, With<Movement>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).remove::<Movement>();
    }
}

pub fn fade_animation_system(
    mut query: Query<(&mut Transform, &Fade), With<Paddle>>,
) {
    // Grow/Shrink paddles to show/hide them
    for (mut transform, fade) in query.iter_mut() {
        transform.scale = *PADDLE_SCALE * fade.opacity();
    }
}

pub fn bounded_movement_system(
    mut query: Query<
        (&mut Transform, &mut Movement),
        (With<Paddle>, With<Active>),
    >,
) {
    for (mut transform, mut movement) in query.iter_mut() {
        // Limit paddle to open space between barriers
        if transform.translation.x > PADDLE_MAX_POSITION_X {
            transform.translation.x = PADDLE_MAX_POSITION_X;
            movement.speed = 0.0;
            movement.delta = None;
        } else if transform.translation.x < -PADDLE_MAX_POSITION_X {
            transform.translation.x = -PADDLE_MAX_POSITION_X;
            movement.speed = 0.0;
            movement.delta = None;
        }
    }
}
