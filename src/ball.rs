use bevy::prelude::*;

use crate::{
    goal::{Barrier, Wall, BARRIER_RADIUS, WALL_RADIUS},
    movement::Heading,
    paddle::{Paddle, PADDLE_HALF_DEPTH, PADDLE_HALF_WIDTH},
    side::Side,
    spawning::{Despawning, Spawning},
    state::AppState,
};

pub const BALL_DIAMETER: f32 = 0.08;
pub const BALL_HEIGHT: f32 = 0.05;
pub const BALL_RADIUS: f32 = 0.5 * BALL_DIAMETER;

/// Marks a ball entity that can collide and score.
#[derive(Component, Debug)]
pub struct Ball;

#[derive(SystemSet, Clone, Hash, Debug, PartialEq, Eq)]
pub struct BallSet;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            PostUpdate,
            BallSet.run_if(in_state(AppState::Playing)),
        )
        .add_systems(
            PostUpdate,
            (
                ball_to_ball_collisions,
                ball_to_barrier_collisions,
                ball_to_paddle_collisions,
                ball_to_wall_collisions,
            )
                .in_set(BallSet),
        );
    }
}

fn reflect(d: Vec3, n: Vec3) -> Vec3 {
    (d - (2.0 * (d.dot(n) * n))).normalize()
}

fn ball_to_ball_collisions(
    mut commands: Commands,
    balls_query: Query<
        (Entity, &GlobalTransform, &Heading),
        (With<Ball>, Without<Spawning>, Without<Despawning>),
    >,
) {
    for (entity, ball_transform, ball_heading) in &balls_query {
        for (entity2, transform2, _) in &balls_query {
            // Prevent balls from colliding with themselves.
            if entity == entity2 {
                continue;
            }

            let ball_to_ball_distance = ball_transform
                .translation()
                .distance(transform2.translation());
            let axis = (transform2.translation()
                - ball_transform.translation())
            .normalize();

            // Check that the ball is touching the other ball and facing it.
            if ball_to_ball_distance > 2.0 * BALL_RADIUS
                || ball_heading.0.dot(axis) <= 0.0
            {
                continue;
            }

            // Deflect the ball away from the other ball.
            commands
                .entity(entity)
                .insert(Heading(reflect(ball_heading.0, axis)));

            info!("Ball({:?}): Collided Ball({:?})", entity, entity2);
            break;
        }
    }
}

fn ball_to_barrier_collisions(
    mut commands: Commands,
    balls_query: Query<
        (Entity, &GlobalTransform, &Heading),
        (With<Ball>, Without<Spawning>, Without<Despawning>),
    >,
    barriers_query: Query<&GlobalTransform, With<Barrier>>,
) {
    for (entity, ball_transform, ball_heading) in &balls_query {
        for barrier_transform in &barriers_query {
            let ball_to_barrier_distance = ball_transform
                .translation()
                .distance(barrier_transform.translation());

            // Prevent balls from deflecting through the floor.
            let mut axis =
                barrier_transform.translation() - ball_transform.translation();

            axis.y = 0.0;
            axis = axis.normalize();

            // Check that the ball is touching the barrier and facing it.
            if ball_to_barrier_distance > BARRIER_RADIUS + BALL_RADIUS
                || ball_heading.0.dot(axis) <= 0.0
            {
                continue;
            }

            // Deflect the ball away from the barrier.
            commands
                .entity(entity)
                .insert(Heading(reflect(ball_heading.0, axis)));

            info!("Ball({:?}): Collided Barrier", entity);
            break;
        }
    }
}

fn ball_to_paddle_collisions(
    mut commands: Commands,
    balls_query: Query<
        (Entity, &GlobalTransform, &Heading),
        (With<Ball>, Without<Spawning>, Without<Despawning>),
    >,
    paddles_query: Query<
        (&Side, &Transform),
        (With<Paddle>, Without<Despawning>),
    >,
) {
    for (entity, ball_transform, ball_heading) in &balls_query {
        for (side, transform) in &paddles_query {
            let goal_axis = side.axis();
            let ball_distance_to_goal = side.distance_to_ball(ball_transform);
            let ball_goal_position = side.get_ball_position(ball_transform);
            let ball_to_paddle = transform.translation.x - ball_goal_position;
            let ball_distance_to_paddle = ball_to_paddle.abs();

            // Check that the ball is touching the paddle and facing the goal.
            if ball_distance_to_goal > PADDLE_HALF_DEPTH
                || ball_distance_to_paddle >= PADDLE_HALF_WIDTH
                || ball_heading.0.dot(goal_axis) <= 0.0
            {
                continue;
            }

            // Reverse the ball's direction and rotate it outward based on how
            // far its position is from the paddle's center.
            let rotation_away_from_center = Quat::from_rotation_y(
                std::f32::consts::FRAC_PI_4
                    * (ball_to_paddle / PADDLE_HALF_WIDTH),
            );
            commands
                .entity(entity)
                .insert(Heading(rotation_away_from_center * -ball_heading.0));

            info!("Ball({:?}): Collided Paddle({:?})", entity, side);
            break;
        }
    }
}

fn ball_to_wall_collisions(
    mut commands: Commands,
    balls_query: Query<
        (Entity, &GlobalTransform, &Heading),
        (With<Ball>, Without<Spawning>, Without<Despawning>),
    >,
    walls_query: Query<&Side, (With<Wall>, Without<Despawning>)>,
) {
    for (entity, ball_transform, ball_heading) in &balls_query {
        for side in &walls_query {
            let ball_distance = side.distance_to_ball(ball_transform);
            let axis = side.axis();

            // Check that the ball is touching and facing the wall.
            if ball_distance > WALL_RADIUS || ball_heading.0.dot(axis) <= 0.0 {
                continue;
            }

            // Deflect the ball away from the wall.
            commands
                .entity(entity)
                .insert(Heading(reflect(ball_heading.0, axis)));

            info!("Ball({:?}): Collided Wall({:?})", entity, side);
            break;
        }
    }
}

// TODO: Debug option to directly control single ball's exact position with
// keyboard and see how paddles respond. Can go in goals, triggering a score and
// ball return?

// TODO: Need a fix for the rare occasion when a ball just bounces infinitely
// between two walls in a straight line? Maybe make all bounces slightly adjust
// ball angle rather than pure reflection?
