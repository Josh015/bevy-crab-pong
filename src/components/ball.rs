use bevy::prelude::*;

/// Marks a ball entity that can collide and score.
#[derive(Component, Debug)]
pub struct Ball;

// TODO: Debug option to directly control single ball's exact position with
// keyboard and see how crabs respond. Can go in goals, triggering a score and
// ball return?
