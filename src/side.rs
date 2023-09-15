use bevy::prelude::{Component, GlobalTransform, Vec3};
use serde::Deserialize;

use crate::goal::GOAL_WIDTH;

pub const SIDES: [Side; 4] = [Side::Bottom, Side::Right, Side::Top, Side::Left];

/// Assigns an entity to a given side of the beach.
#[derive(Clone, Component, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
pub enum Side {
    Bottom = 0,
    Right = 1,
    Top = 2,
    Left = 3,
}

impl Side {
    /// Perpendicular distance from a given goal to a ball's center.
    ///
    /// Positive distances for inside the beach, negative for out of bounds.
    pub fn distance_to_ball(&self, ball_transform: &GlobalTransform) -> f32 {
        let ball_translation = ball_transform.translation();

        match *self {
            Self::Bottom => (0.5 * GOAL_WIDTH) - ball_translation.z,
            Self::Right => (0.5 * GOAL_WIDTH) - ball_translation.x,
            Self::Top => (0.5 * GOAL_WIDTH) + ball_translation.z,
            Self::Left => (0.5 * GOAL_WIDTH) + ball_translation.x,
        }
    }

    /// Get the (+/-)(X/Z) axis the side occupies.
    pub fn axis(&self) -> Vec3 {
        match *self {
            Self::Bottom => Vec3::Z,
            Self::Right => Vec3::X,
            Self::Top => -Vec3::Z,
            Self::Left => -Vec3::X,
        }
    }

    /// Map a ball's global position to a side's local x-axis.
    pub fn get_ball_position(&self, ball_transform: &GlobalTransform) -> f32 {
        match *self {
            Self::Bottom => ball_transform.translation().x,
            Self::Right => -ball_transform.translation().z,
            Self::Top => -ball_transform.translation().x,
            Self::Left => ball_transform.translation().z,
        }
    }
}
