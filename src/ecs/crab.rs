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
pub struct Crab {
    pub movement: Movement,
    pub speed: f32,
}

pub fn step_fade_animation_system(
    config: Res<GameConfig>,
    mut query: Query<(&mut Transform, &Fade), With<Crab>>,
) {
    // Grow/Shrink crabs to show/hide them
    for (mut transform, fade) in query.iter_mut() {
        transform.scale = config.crab_scale.into();
        transform.scale *= fade.opacity();
    }
}

pub fn movement_system(
    config: Res<GameConfig>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Crab), With<Active>>,
) {
    for (mut transform, mut crab) in query.iter_mut() {
        // Accelerate the crab
        let delta_speed = config.crab_acceleration() * time.delta_seconds();

        if crab.movement == Movement::Stopped {
            let s = crab.speed.abs().sub(delta_speed).max(0.0);
            crab.speed = crab.speed.max(-s).min(s);
        } else {
            crab.speed = crab
                .speed
                .add(if crab.movement == Movement::Left {
                    -delta_speed
                } else {
                    delta_speed
                })
                .clamp(-config.crab_max_speed, config.crab_max_speed);
        }

        // Limit crab to open space between barriers
        let mut position = transform.translation.x + crab.speed;
        let extents = 0.5
            * (config.beach_width - config.barrier_width - config.crab_scale.0);

        if position >= extents {
            position = extents;
            crab.speed = 0.0;
        } else if position <= -extents {
            position = -extents;
            crab.speed = 0.0;
        }

        // Move the crab
        transform.translation.x = position;
    }
}
