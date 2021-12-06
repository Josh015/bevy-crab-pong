use super::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

pub fn paddle_control_system(
    mut paddles_query: Query<
        (&Transform, &Goal, &mut Paddle, &Velocity),
        (With<Active>, With<Enemy>),
    >,
    balls_query: Query<&GlobalTransform, (With<Ball>, With<Active>)>,
) {
    for (transform, goal, mut paddle, velocity) in paddles_query.iter_mut() {
        // Get the relative position of the ball that's closest to this goal
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

        // Predict the paddle's stop position if it begins decelerating now
        let d = velocity.speed * velocity.speed / velocity.acceleration;
        let stop_position = if velocity.speed > 0.0 {
            transform.translation.x + d
        } else {
            transform.translation.x - d
        };

        // Position the paddle so that the ball is above ~70% of its center
        let distance_from_paddle_center =
            (stop_position - target_position).abs();

        if distance_from_paddle_center < 0.7 * PADDLE_HALF_WIDTH {
            *paddle = Paddle::Stop;
        } else if target_position < transform.translation.x {
            *paddle = Paddle::Left;
        } else {
            *paddle = Paddle::Right;
        }
    }
}
