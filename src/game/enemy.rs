use bevy::prelude::{Component, GlobalTransform, Query, Res, Transform, With};

use crate::GameConfig;

use super::{ball::Ball, fade::Active, goal::Goal, paddle::Paddle, Velocity};

#[derive(Component)]
pub struct Enemy;

pub fn paddle_control_system(
    config: Res<GameConfig>,
    balls_query: Query<&GlobalTransform, (With<Ball>, With<Active>)>,
    mut paddles_query: Query<
        (&Transform, &GlobalTransform, &Goal, &mut Paddle, &Velocity),
        (With<Active>, With<Enemy>),
    >,
) {
    for (local, global, goal, mut paddle, velocity) in paddles_query.iter_mut()
    {
        // Pick which ball is closest to this paddle's goal
        let mut closest_ball_distance = std::f32::MAX;
        let mut target_position = config.paddle_start_position.0;

        for ball_transform in balls_query.iter() {
            // Remap from ball's global space to paddle's local space
            let ball_translation = ball_transform.translation;
            let ball_distance = global.translation.distance(ball_translation);
            let ball_position = match *goal {
                Goal::Top => -ball_translation.x,
                Goal::Right => -ball_translation.z,
                Goal::Bottom => ball_translation.x,
                Goal::Left => ball_translation.z,
            };

            if ball_distance < closest_ball_distance {
                target_position = ball_position;
                closest_ball_distance = ball_distance;
            }
        }

        // Predict the position where the paddle will stop if it immediately
        // begins decelerating.
        let d = velocity.speed * velocity.speed / velocity.acceleration;
        let stop_position = if velocity.speed > 0.0 {
            local.translation.x + d
        } else {
            local.translation.x - d
        };

        // Begin decelerating if the ball will land over 70% of the paddle's
        // middle at its predicted stop position. Otherwise go left/right
        // depending on which side of the paddle it's approaching.
        if (stop_position - target_position).abs()
            < 0.7 * (config.paddle_scale.0 * 0.5)
        {
            *paddle = Paddle::Stop;
        } else if target_position < local.translation.x {
            *paddle = Paddle::Left;
        } else {
            *paddle = Paddle::Right;
        }
    }
}
