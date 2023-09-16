use bevy::prelude::*;

pub const WALL_DIAMETER: f32 = 0.05;
pub const WALL_HEIGHT: f32 = 0.1;
pub const WALL_RADIUS: f32 = 0.5 * WALL_DIAMETER;

/// Makes an entity a wall that deflects all balls away from a goal.
#[derive(Component, Debug)]
pub struct Wall;
