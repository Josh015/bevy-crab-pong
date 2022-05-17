use crate::prelude::*;

pub const WALL_DIAMETER: f32 = 0.05;
pub const WALL_HEIGHT: f32 = 0.1;
pub const WALL_RADIUS: f32 = 0.5 * WALL_DIAMETER;
pub const WALL_SCALE: Vec3 =
    const_vec3!([GOAL_WIDTH, WALL_DIAMETER, WALL_DIAMETER]);

/// A component that makes an entity a wall in a `Goal` that can deflect `Ball`
/// entities away from the entire goal when `Collider`.
#[derive(Component)]
pub struct Wall {
    pub goal_side: GoalSide,
}
