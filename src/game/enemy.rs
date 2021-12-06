use super::*;
use crate::GameConfig;
use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

pub fn paddle_control_system(
    config: Res<GameConfig>,
    mut paddles_query: Query<
        (&Transform, &Goal, &mut Paddle, &Velocity),
        (With<Active>, With<Enemy>),
    >,
    balls_query: Query<&GlobalTransform, (With<Ball>, With<Active>)>,
) {
    for (transform, goal, mut paddle, velocity) in paddles_query.iter_mut() {
        // Pick which ball is closest to this paddle's goal
        let mut closest_ball_distance = std::f32::MAX;
        let mut target_position = config.paddle_start_position.0;

        for ball_transform in balls_query.iter() {
            // Remap from ball's global space to paddle's local space
            let ball_distance = goal.distance_to_ball(&config, ball_transform);
            let ball_position = goal.map_ball_to_paddle_axis(ball_transform);

            if ball_distance < closest_ball_distance {
                target_position = ball_position;
                closest_ball_distance = ball_distance;
            }
        }

        // Predict the position where the paddle will stop if it immediately
        // begins decelerating.
        let d = velocity.speed * velocity.speed / velocity.acceleration;
        let stop_position = if velocity.speed > 0.0 {
            transform.translation.x + d
        } else {
            transform.translation.x - d
        };

        // Begin decelerating if the ball will land within 70% of the paddle's
        // middle at its predicted stop position. Otherwise go left/right
        // depending on which side of the paddle it's approaching.
        let distance_from_paddle_center =
            (stop_position - target_position).abs();

        if distance_from_paddle_center < 0.7 * (config.paddle_scale.0 * 0.5) {
            *paddle = Paddle::Stop;
        } else if target_position < transform.translation.x {
            *paddle = Paddle::Left;
        } else {
            *paddle = Paddle::Right;
        }
    }
}
