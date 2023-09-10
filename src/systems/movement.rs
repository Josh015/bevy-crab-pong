use bevy::prelude::*;
use std::ops::{Add, Sub};

use crate::{
    components::{
        movement::{
            Acceleration, Force, Heading, MaxSpeed, Speed, StoppingDistance,
        },
        paddles::Paddle,
        spawning::Spawning,
    },
    constants::*,
};

use super::GameSystemSet;

fn decelerate_speed(speed: f32, delta_speed: f32) -> f32 {
    let s = speed.abs().sub(delta_speed).max(0.0);
    speed.max(-s).min(s) // clamp() panics when min == max.
}

fn acceleration(
    time: Res<Time>,
    mut query: Query<
        (&mut Speed, &Acceleration, &Force, &MaxSpeed),
        Without<Spawning>,
    >,
) {
    for (mut speed, acceleration, force, max_speed) in &mut query {
        let delta_speed = acceleration.0 * time.delta_seconds();

        speed.0 = speed
            .0
            .add(if *force == Force::Positive {
                delta_speed
            } else {
                -delta_speed
            })
            .clamp(-max_speed.0, max_speed.0);
    }
}

fn deceleration(
    time: Res<Time>,
    mut query: Query<
        (&mut Speed, &Acceleration),
        (Without<Force>, Without<Spawning>),
    >,
) {
    for (mut speed, acceleration) in &mut query {
        let delta_speed = acceleration.0 * time.delta_seconds();
        speed.0 = decelerate_speed(speed.0, delta_speed);
    }
}

fn velocity(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Heading, &Speed), Without<Spawning>>,
) {
    for (mut transform, heading, speed) in &mut query {
        transform.translation += heading.0 * (speed.0 * time.delta_seconds());
    }
}

fn stopping_distance(
    mut query: Query<
        (&mut StoppingDistance, &Acceleration, &Speed),
        Without<Spawning>,
    >,
) {
    for (mut stopping_distance, acceleration, speed) in &mut query {
        const DELTA_SECONDS: f32 = 0.01;
        let delta_speed = acceleration.0 * DELTA_SECONDS;
        let mut current_speed = speed.0;

        stopping_distance.0 = 0f32;

        while current_speed.abs() > 0.0 {
            stopping_distance.0 += current_speed * DELTA_SECONDS;
            current_speed = decelerate_speed(current_speed, delta_speed);
        }
    }
}

fn restrict_paddles_to_open_space_in_their_goals(
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut Transform, &mut Speed, &mut StoppingDistance),
        (With<Paddle>, Without<Spawning>),
    >,
) {
    for (entity, mut transform, mut speed, mut stopping_distance) in &mut query
    {
        // Limit paddle to bounds of the goal.
        if !GOAL_PADDLE_MAX_POSITION_RANGE.contains(&transform.translation.x) {
            transform.translation.x = transform
                .translation
                .x
                .clamp(-GOAL_PADDLE_MAX_POSITION_X, GOAL_PADDLE_MAX_POSITION_X);
            speed.0 = 0.0;
            commands.entity(entity).remove::<Force>();
        }

        // Limit stopping distance to the bounds of the goal.
        let stopped_position = transform.translation.x + stopping_distance.0;

        if !GOAL_PADDLE_MAX_POSITION_RANGE.contains(&stopped_position) {
            stopping_distance.0 = stopped_position.signum()
                * GOAL_PADDLE_MAX_POSITION_X
                - transform.translation.x;
        }
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                acceleration,
                deceleration,
                velocity,
                stopping_distance,
                restrict_paddles_to_open_space_in_their_goals,
            )
                .chain()
                .in_set(GameSystemSet::Movement),
        );
    }
}
