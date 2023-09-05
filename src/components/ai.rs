#![allow(clippy::type_complexity)]

use crate::prelude::*;

/// A component that works in tandem with [`Paddle`] to make AI-driven
/// opponents.
#[derive(Component)]
pub struct Ai;

/// AI control for [`Paddle`] entities.
fn ai_paddle_control(
    mut commands: Commands,
    paddles_query: Query<
        (Entity, &Side, &Transform, &Speed, &Acceleration, &Target),
        (With<Paddle>, With<Ai>),
    >,
    balls_query: Query<&GlobalTransform, (With<Ball>, With<Collider>)>,
) {
    // We want the paddle to follow and try to stay under the moving ball
    // rather than going straight to where it will cross the goal.
    for (entity, side, transform, speed, acceleration, target) in &paddles_query
    {
        // Get the targeted ball's position in the goal's local space.
        let Ok(target) = balls_query.get(target.0) else {
            continue;
        };
        let ball_local_position =
            side.map_ball_position_to_paddle_local_space(target);

        // Predict the paddle's stop position if it begins decelerating now.
        const DELTA_SECONDS: f32 = 0.05; // Overshoots the ball slightly more often.
        // const DELTA_SECONDS: f32 = 0.001; // Precisely follows the ball.
        let delta_speed = acceleration.0 * DELTA_SECONDS;
        let mut current_speed = speed.0;
        let mut paddle_stop_position = transform.translation.x;

        while current_speed.abs() > 0.0 {
            paddle_stop_position += current_speed * DELTA_SECONDS;
            current_speed = decelerate_speed(current_speed, delta_speed);
        }

        // Controls how much the paddle tries to get its center under the ball.
        // Lower values improve the catch rate, but also reduce how widely it
        // will deflect the ball for near misses. Range (0,1].
        const PERCENT_FROM_CENTER: f32 = 0.60;
        let distance_from_paddle_center =
            (paddle_stop_position - ball_local_position).abs();

        if distance_from_paddle_center < PERCENT_FROM_CENTER * PADDLE_HALF_WIDTH
        {
            commands.entity(entity).remove::<Force>();
        } else {
            commands.entity(entity).insert(
                if ball_local_position < transform.translation.x {
                    Force::Negative // Left
                } else {
                    Force::Positive // Right
                },
            );
        }
    }
}

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            ai_paddle_control.in_set(GameSystemSet::GameplayLogic),
        );
    }
}
