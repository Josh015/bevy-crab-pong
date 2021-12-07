use super::*;
use bevy::prelude::*;

/// A component that works in tandem with `Paddle` to make AI-driven opponents.
#[derive(Component)]
pub struct Enemy;

/// Applies AI control to `Paddle` entities, causing them to position
/// themselves between their `Goal` and the closest `Ball`.
pub fn ai_paddle_control_system(
    mut paddles_query: Query<
        (&Transform, &Goal, &mut Movement),
        (With<Enemy>, With<Paddle>, With<Active>),
    >,
    balls_query: Query<&GlobalTransform, (With<Ball>, With<Active>)>,
) {
    for (transform, goal, mut movement) in paddles_query.iter_mut() {
        // Get the relative position of the ball that's closest to this goal.
        let mut closest_ball_distance = std::f32::MAX;
        let mut target_position = PADDLE_START_POSITION.x;

        for ball_transform in balls_query.iter() {
            let ball_distance = goal.distance_to_ball(ball_transform);

            if ball_distance < closest_ball_distance {
                closest_ball_distance = ball_distance;
                target_position =
                    goal.map_ball_position_to_paddle_range(ball_transform);
            }
        }

        // Predict the paddle's stop position if it begins decelerating now.
        let d = movement.speed * movement.speed / movement.acceleration;
        let stop_position = if movement.speed > 0.0 {
            transform.translation.x + d
        } else {
            transform.translation.x - d
        };

        // Position the paddle so that the ball is above its middle 70%.
        let distance_from_paddle_center =
            (stop_position - target_position).abs();

        movement.delta =
            if distance_from_paddle_center < 0.7 * PADDLE_HALF_WIDTH {
                None
            } else if target_position < transform.translation.x {
                Some(Delta::Negative) // Left
            } else {
                Some(Delta::Positive) // Right
            };
    }
}
