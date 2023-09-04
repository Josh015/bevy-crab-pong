#![allow(clippy::type_complexity)]

use crate::prelude::*;

#[derive(Component)]
pub struct Targeting(pub Entity);

fn detect_and_target_closest_ball(
    mut commands: Commands,
    paddles_query: Query<(Entity, &Side), With<Paddle>>,
    balls_query: Query<
        (Entity, &GlobalTransform),
        (With<Ball>, With<Collider>),
    >,
) {
    // Target the ball that's closest to the paddle's goal.
    for (paddle_entity, side) in &paddles_query {
        let mut closest_ball_distance = std::f32::MAX;
        let mut target = None;

        for (ball_entity, ball_transform) in &balls_query {
            let ball_distance_to_goal = side.distance_to_ball(ball_transform);

            if ball_distance_to_goal < closest_ball_distance {
                closest_ball_distance = ball_distance_to_goal;
                target = Some(ball_entity);
            }
        }

        let Some(target) = target else { continue };
        commands.entity(paddle_entity).insert(Targeting(target));
    }
}

pub struct TargetingPlugin;

impl Plugin for TargetingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            detect_and_target_closest_ball
                .run_if(in_state(GameScreen::Playing)),
        );
    }
}
