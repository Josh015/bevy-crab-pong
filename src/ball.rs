use bevy::prelude::*;

use crate::{
    debug_mode::{DebugModeSet, DEBUGGING_RAY_LENGTH},
    movement::{Heading, Movement},
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
            display_movement_direction_gizmos.in_set(DebugModeSet),
        );
    }
}

fn display_movement_direction_gizmos(
    balls_query: Query<
        (&GlobalTransform, &Heading),
        (With<Ball>, With<Movement>),
    >,
    mut gizmos: Gizmos,
) {
    for (global_transform, heading) in &balls_query {
        gizmos.line(
            global_transform.translation(),
            global_transform.translation() + heading.0 * DEBUGGING_RAY_LENGTH,
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
