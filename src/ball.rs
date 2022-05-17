use crate::prelude::*;

pub const BALL_DIAMETER: f32 = 0.08;
pub const BALL_HEIGHT: f32 = 0.05;
pub const BALL_RADIUS: f32 = 0.5 * BALL_DIAMETER;

/// A component for a ball entity that must have inertia and be able to deflect
/// upon collision when `Collider`.
#[derive(Component)]
pub struct Ball;
