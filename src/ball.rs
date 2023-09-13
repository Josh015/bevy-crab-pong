use bevy::prelude::*;

use crate::{
    collider::{Collider, ColliderSet},
    movement::{Heading, Movement},
    util::reflect,
};

pub const BALL_DIAMETER: f32 = 0.08;
pub const BALL_HEIGHT: f32 = 0.05;
pub const BALL_RADIUS: f32 = 0.5 * BALL_DIAMETER;

/// Marks a ball entity that can collide and score.
#[derive(Component, Debug)]
pub struct Ball;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            ball_and_ball_collisions.in_set(ColliderSet),
        );
    }
}

fn ball_and_ball_collisions(
    mut commands: Commands,
    balls_query: Query<
        (Entity, &GlobalTransform, &Heading),
        (With<Ball>, With<Movement>, With<Collider>),
    >,
) {
    for (entity, ball_transform, ball_heading) in &balls_query {
        for (entity2, transform2, _) in &balls_query {
            // Prevent balls from colliding with themselves.
            if entity == entity2 {
                continue;
            }

            let ball_to_ball_distance = ball_transform
                .translation()
                .distance(transform2.translation());
            let axis = (transform2.translation()
                - ball_transform.translation())
            .normalize();

            // Check that the ball is touching the other ball and facing it.
            if ball_to_ball_distance > 2.0 * BALL_RADIUS
                || ball_heading.0.dot(axis) <= 0.0
            {
                continue;
            }

            // Deflect the ball away from the other ball.
            commands
                .entity(entity)
                .insert(Heading(reflect(ball_heading.0, axis)));

            info!("Ball({:?}): Collided Ball({:?})", entity, entity2);
            break;
        }
    }
}

// TODO: Debug option to directly control single ball's exact position with
// keyboard and see how crabs respond. Can go in goals, triggering a score and
// ball return?

// TODO: Need a fix for the rare occasion when a ball just bounces infinitely
// between two walls in a straight line? Maybe make all bounces slightly adjust
// ball angle rather than pure reflection?
