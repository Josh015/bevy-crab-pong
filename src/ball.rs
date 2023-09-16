use bevy::prelude::*;

pub const BALL_DIAMETER: f32 = 0.08;
pub const BALL_HEIGHT: f32 = 0.05;
pub const BALL_RADIUS: f32 = 0.5 * BALL_DIAMETER;

/// Marks a ball entity that can collide and score.
#[derive(Component, Debug)]
pub struct Ball;

// TODO: Debug option to directly control single ball's exact position with
// keyboard and see how crabs respond. Can go in goals, triggering a score and
// ball return?
