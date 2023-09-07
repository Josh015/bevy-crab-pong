use crate::prelude::*;

/// Assigns and entity to a given side of the arena.
#[derive(Clone, Component, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

impl Side {
    /// Perpendicular distance from a given goal to a ball's edge.
    ///
    /// Positive distances for inside the arena, negative for out of bounds.
    pub fn distance_to_ball(&self, ball_transform: &GlobalTransform) -> f32 {
        let ball_translation = ball_transform.translation();

        match *self {
            Self::Top => GOAL_HALF_WIDTH + ball_translation.z - BALL_RADIUS,
            Self::Right => GOAL_HALF_WIDTH - ball_translation.x - BALL_RADIUS,
            Self::Bottom => GOAL_HALF_WIDTH - ball_translation.z - BALL_RADIUS,
            Self::Left => GOAL_HALF_WIDTH + ball_translation.x - BALL_RADIUS,
        }
    }

    /// Get the (+/-)(X/Z) axis the side occupies.
    pub fn axis(&self) -> Vec3 {
        match *self {
            Self::Top => -Vec3::Z,
            Self::Right => Vec3::X,
            Self::Bottom => Vec3::Z,
            Self::Left => -Vec3::X,
        }
    }

    /// Map a ball's global position to a side's local x-axis.
    pub fn get_ball_position(&self, ball_transform: &GlobalTransform) -> f32 {
        match *self {
            Self::Top => -ball_transform.translation().x,
            Self::Right => -ball_transform.translation().z,
            Self::Bottom => ball_transform.translation().x,
            Self::Left => ball_transform.translation().z,
        }
    }
}
