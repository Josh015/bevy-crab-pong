#![allow(clippy::type_complexity)]

use crate::prelude::*;

/// Makes a paddle try to get the middle 50% of its width under the ball.
const PADDLE_CENTER_HIT_AREA_PERCENTAGE: f32 = 0.5;

/// A component that works in tandem with [`Paddle`] to make AI-driven
/// opponents.
#[derive(Component)]
pub struct AiControlled;

/// A component that works with [`Ai`] to target, follow, and try to block one
/// of the balls.
#[derive(Component)]
pub struct Target(pub Entity);

/// AI control for [`Paddle`] entities.
fn ai_controlled_paddles(
    mut commands: Commands,
    paddles_query: Query<
        (
            Entity,
            &Side,
            &Transform,
            &StoppingDistance,
            Option<&Target>,
        ),
        (With<AiControlled>, With<Paddle>),
    >,
    balls_query: Query<&GlobalTransform, (With<Ball>, With<Collider>)>,
) {
    for (entity, side, transform, stopping_distance, target) in &paddles_query {
        // Use the ball's goal position or default to the center of the goal.
        let mut target_goal_position = ARENA_CENTER_POINT.x;

        if let Some(target) = target {
            if let Ok(ball_transform) = balls_query.get(target.0) {
                target_goal_position = side.get_ball_position(ball_transform)
            }
        }

        // Move the paddle so that its center is over the target goal position.
        let paddle_stop_position =
            transform.translation.x + stopping_distance.0;
        let distance_from_paddle_center =
            (paddle_stop_position - target_goal_position).abs();

        if distance_from_paddle_center
            < PADDLE_CENTER_HIT_AREA_PERCENTAGE * PADDLE_HALF_WIDTH
        {
            commands.entity(entity).remove::<Force>();
        } else {
            commands.entity(entity).insert(
                if target_goal_position < transform.translation.x {
                    Force::Negative // Left
                } else {
                    Force::Positive // Right
                },
            );
        }
    }
}

/// Causes [`Ai`] entities to target whichever ball is closest to their goal.
fn detect_and_target_ball_closest_to_goal(
    mut commands: Commands,
    paddles_query: Query<(Entity, &Side), (With<AiControlled>, With<Paddle>)>,
    balls_query: Query<
        (Entity, &GlobalTransform),
        (With<Ball>, With<Collider>),
    >,
) {
    for (ai_entity, side) in &paddles_query {
        let mut closest_ball_distance = std::f32::MAX;
        let mut target = None;

        for (ball_entity, ball_transform) in &balls_query {
            let ball_distance_to_goal = side.distance_to_ball(ball_transform);

            if ball_distance_to_goal < closest_ball_distance {
                closest_ball_distance = ball_distance_to_goal;
                target = Some(ball_entity);
            }
        }

        if let Some(target) = target {
            commands.entity(ai_entity).insert(Target(target));
        } else {
            commands.entity(ai_entity).remove::<Target>();
        }
    }
}

/// Provides debug visualization to show which [`Ai`] entities are targeting
/// which [`Ball`] entities.
fn debug_targeting(
    paddles_query: Query<
        (&GlobalTransform, &Target),
        (With<AiControlled>, With<Paddle>, Without<Fade>),
    >,
    balls_query: Query<&GlobalTransform, (With<Ball>, With<Collider>)>,
    mut gizmos: Gizmos,
) {
    for (paddle_transform, target) in &paddles_query {
        if let Ok(ball_transform) = balls_query.get(target.0) {
            gizmos.line(
                paddle_transform.translation(),
                ball_transform.translation(),
                Color::PURPLE,
            );
        }
    }
}

/// Provides debug visualization to show the size of the ideal hit area on each
/// [`Ai`] [`Paddle`] entity.
fn debug_ai_paddle_hit_area(
    paddles_query: Query<
        &GlobalTransform,
        (With<Paddle>, With<AiControlled>, Without<Fade>),
    >,
    mut gizmos: Gizmos,
) {
    for global_transform in &paddles_query {
        let mut hit_area_transform = global_transform.compute_transform();

        hit_area_transform.scale.x =
            PADDLE_CENTER_HIT_AREA_PERCENTAGE * PADDLE_WIDTH;
        gizmos.cuboid(hit_area_transform, Color::YELLOW);
    }
}

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (
                    detect_and_target_ball_closest_to_goal,
                    ai_controlled_paddles,
                )
                    .chain()
                    .in_set(GameSystemSet::GameplayLogic),
                (debug_targeting, debug_ai_paddle_hit_area)
                    .in_set(GameSystemSet::Debugging),
            ),
        );
    }
}
