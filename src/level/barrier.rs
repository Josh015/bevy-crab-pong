use bevy::prelude::*;

use crate::{
    common::{
        collider::{Collider, ColliderSet},
        movement::{Heading, Movement},
    },
    object::ball::{Ball, BALL_RADIUS},
    util::reflect,
};

pub const BARRIER_DIAMETER: f32 = 0.12;
pub const BARRIER_RADIUS: f32 = 0.5 * BARRIER_DIAMETER;
pub const BARRIER_HEIGHT: f32 = 0.2;

/// Marks an entity as a barrier to deflect all balls away from a corner.
#[derive(Component, Debug)]
pub struct Barrier;

pub(super) struct BarrierPlugin;

impl Plugin for BarrierPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            barrier_and_ball_collisions.in_set(ColliderSet),
        );
    }
}

fn barrier_and_ball_collisions(
    mut commands: Commands,
    balls_query: Query<
        (Entity, &GlobalTransform, &Heading),
        (With<Ball>, With<Collider>, With<Movement>),
    >,
    barriers_query: Query<
        (Entity, &GlobalTransform),
        (With<Barrier>, With<Collider>),
    >,
) {
    for (ball_entity, ball_transform, ball_heading) in &balls_query {
        for (barrier_entity, barrier_transform) in &barriers_query {
            // Prevent balls from deflecting through the floor.
            let delta =
                barrier_transform.translation() - ball_transform.translation();
            let mut axis = delta;

            axis.y = 0.0;
            axis = axis.normalize();

            // Check that the ball is touching the barrier and facing it.
            if delta.length() > BARRIER_RADIUS + BALL_RADIUS
                || ball_heading.0.dot(axis) <= 0.0
            {
                continue;
            }

            // Deflect the ball away from the barrier.
            commands
                .entity(ball_entity)
                .insert(Heading(reflect(ball_heading.0, axis).normalize()));

            info!(
                "Ball({:?}): Collided Barrier({:?})",
                ball_entity, barrier_entity
            );
            break;
        }
    }
}
