use crate::prelude::*;

pub const PADDLE_WIDTH: f32 = 0.2;
pub const PADDLE_DEPTH: f32 = 0.1;
pub const PADDLE_HALF_WIDTH: f32 = 0.5 * PADDLE_WIDTH;
pub const PADDLE_HALF_DEPTH: f32 = 0.5 * PADDLE_DEPTH;
pub const PADDLE_SCALE: Vec3 =
    const_vec3!([PADDLE_WIDTH, PADDLE_DEPTH, PADDLE_DEPTH]);

/// A component that makes a paddle that can deflect `Ball` entities and moves
/// left->right and vice versa along a single axis when `Collider`.
#[derive(Clone, Component, Eq, PartialEq, Debug, Hash)]
pub struct Paddle {
    pub goal_side: GoalSide,
}

/// Handles the `Fade` animation for a `Paddle` entity by causing it to
/// grow/shrink into/out of existence.
pub fn paddle_fade_animation_system(
    mut query: Query<(&mut Transform, &Fade), With<Paddle>>,
) {
    // Grow/Shrink paddles to show/hide them
    for (mut transform, fade) in query.iter_mut() {
        transform.scale = PADDLE_SCALE * fade.opacity();
    }
}
