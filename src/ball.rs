use bevy::prelude::*;

use crate::{
    collider::{Collider, ColliderSet},
    debug_mode::DebugModeSet,
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
            (
                ball_and_ball_collisions.in_set(ColliderSet),
                display_ball_movement_direction_gizmos.in_set(DebugModeSet),
            ),
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
    for [(entity1, transform1, heading1), (entity2, transform2, heading2)] in
        balls_query.iter_combinations()
    {
        // Check that both balls are close enough to touch.
        let translation1 = transform1.translation();
        let translation2 = transform2.translation();
        let b1_to_b2_vector = translation2 - translation1;

        if b1_to_b2_vector.length() > BALL_RADIUS + BALL_RADIUS {
            continue;
        }

        // Deflect both balls away from each other.
        let axis1 = b1_to_b2_vector.normalize();
        let axis2 = -axis1;
        let is_b1_facing_b2 = heading1.0.dot(axis1) > 0.0;
        let is_b2_facing_b1 = heading2.0.dot(axis2) > 0.0;

        if is_b1_facing_b2 {
            commands
                .entity(entity1)
                .insert(Heading(reflect(heading1.0, axis1)));
        } else if is_b2_facing_b1 {
            commands.entity(entity1).insert(Heading(axis2));
        }

        if is_b2_facing_b1 {
            commands
                .entity(entity2)
                .insert(Heading(reflect(heading2.0, axis2)));
        } else if is_b1_facing_b2 {
            commands.entity(entity2).insert(Heading(axis1));
        }

        info!("Ball({:?}): Collided Ball({:?})", entity1, entity2);
    }
}

fn display_ball_movement_direction_gizmos(
    balls_query: Query<
        (&GlobalTransform, &Heading),
        (With<Ball>, With<Movement>),
    >,
    mut gizmos: Gizmos,
) {
    for (global_transform, heading) in &balls_query {
        gizmos.line(
            global_transform.translation(),
            global_transform.translation() + heading.0 * 20.0,
            Color::RED,
        )
    }
}

// TODO: Debug option to directly control single ball's exact position with
// keyboard and see how crabs respond. Can go in goals, triggering a score and
// ball return?

// TODO: Need a fix for the rare occasion when a ball just bounces infinitely
// between two walls in a straight line? Maybe make all bounces slightly adjust
// ball angle rather than pure reflection?
