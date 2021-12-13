use super::*;
use crate::GameConfig;

/// A component that makes a paddle that can deflect `Ball` entities and moves
/// left->right and vice versa along a single axis when `Active`.
#[derive(Clone, Component, Eq, PartialEq, Debug, Hash)]
pub struct Paddle;

/// Immediately stops `Movement` for a recently deactivated `Paddle` entity.
pub fn stop_when_inactive_system(
    mut query: Query<&mut Movement, (With<Paddle>, Without<Active>)>,
) {
    for mut movement in query.iter_mut() {
        movement.dead_stop();
    }
}

/// Handles the `Fade` animation for a `Paddle` entity by causing it to
/// grow/shrink into/out of existence.
pub fn fade_animation_system(
    mut query: Query<(&mut Transform, &Fade), With<Paddle>>,
) {
    // Grow/Shrink paddles to show/hide them
    for (mut transform, fade) in query.iter_mut() {
        transform.scale = PADDLE_SCALE * fade.opacity();
    }
}

/// Restricts a `Paddle` entity to the space between the `Barrier` entities on
/// either side of it.
pub fn bounded_movement_system(
    mut query: Query<
        (&mut Transform, &mut Movement),
        (With<Paddle>, With<Active>),
    >,
) {
    for (mut transform, mut movement) in query.iter_mut() {
        // Limit paddle to open space between barriers
        if transform.translation.x > PADDLE_MAX_POSITION_X {
            transform.translation.x = PADDLE_MAX_POSITION_X;
            movement.dead_stop();
        } else if transform.translation.x < -PADDLE_MAX_POSITION_X {
            transform.translation.x = -PADDLE_MAX_POSITION_X;
            movement.dead_stop();
        }
    }
}
