use super::*;
use crate::GameConfig;
use bevy::{ecs::prelude::*, prelude::*};
use rand::prelude::*;

/// A component for a ball entity that must have inertia and be able to deflect
/// upon collision when `Active`.
#[derive(Component)]
pub struct Ball;

/// Handles the `Fade` animation for a `Ball` entity by causing its material to
/// smoothly blend from opaque->transparent and vice versa.
pub fn fade_animation_system(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<
        (&mut Fade, &Handle<StandardMaterial>, &mut Transform),
        With<Ball>,
    >,
) {
    let mut is_prior_resetting = false;

    for (mut fade, material, mut transform) in query.iter_mut() {
        let is_current_resetting = matches!(*fade, Fade::In(_));

        // Force current ball to wait if other is also fading in
        if is_prior_resetting && is_current_resetting {
            *fade = Fade::In(0.0);
            continue;
        }

        is_prior_resetting = is_current_resetting;

        // materials
        //     .get_mut(material)
        //     .unwrap()
        //     .base_color
        //     .set_a(fade.opacity());

        // FIXME: Use scaling until we can get alpha-blending working
        transform.scale = Vec3::splat(fade.opacity() * BALL_DIAMETER);
    }
}

/// Takes a fully hidden `Ball`, disables its movement, moves it back to the
/// center of the arena, and then slowly fades it back into view.
pub fn reset_position_system(
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut Transform),
        (With<Ball>, Without<Fade>, Without<Active>),
    >,
) {
    for (entity, mut transform) in query.iter_mut() {
        transform.translation = *BALL_CENTER_POINT;
        commands
            .entity(entity)
            .remove::<Movement>()
            .insert(Fade::In(0.0));
    }
}

/// Takes a newly `Active` `Ball` and gives it `Movement` so that it starts it
/// moving in a straight line in a random direction.
pub fn reset_movement_system(
    mut commands: Commands,
    config: Res<GameConfig>,
    query: Query<Entity, (With<Ball>, Without<Movement>, Added<Active>)>,
) {
    for entity in query.iter() {
        // TODO: Move this into a global resource? Also, make a reusable uniform
        // for random rotation?
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);

        commands.entity(entity).insert(Movement {
            direction: Vec3::new(angle.cos(), 0.0, angle.sin()),
            speed: config.ball_starting_speed,
            max_speed: config.ball_max_speed,
            acceleration: config.ball_max_speed
                / config.ball_seconds_to_max_speed,
            delta: Some(Delta::Positive),
        });
    }
}

/// Checks if a `Ball` has collided with a compatible entity, and then deflects
/// it away from the point of impact.
pub fn collision_system(
    mut commands: Commands,
    balls_query: Query<
        (Entity, &GlobalTransform, &Movement),
        (With<Ball>, With<Active>),
    >,
    paddles_query: Query<(&Transform, &Goal), (With<Paddle>, With<Active>)>,
    barriers_query: Query<&GlobalTransform, With<Barrier>>,
    walls_query: Query<&Goal, (With<Wall>, With<Active>)>,
) {
    for (entity, ball_transform, movement) in balls_query.iter() {
        let ball_direction = movement.direction;

        // TODO: Get these working as one big system, and then try splitting
        // them up into component-specific systems.

        // TODO: Get collision checks working, printing debug messages for each.

        // Ball collisions
        for (entity2, transform2, movement2) in balls_query.iter() {
            // Prevent balls from colliding with themselves.
            if entity == entity2 {
                continue;
            }

            let ball_to_ball_distance =
                ball_transform.translation.distance(transform2.translation);

            // Prevent balls from deflecting through the floor.
            let axis = (transform2.translation - ball_transform.translation)
                .normalize();

            // Check that the ball is touching the barrier and facing it.
            if ball_to_ball_distance > BALL_RADIUS + BALL_RADIUS
                || ball_direction.dot(axis) <= 0.0
            {
                continue;
            }

            // Deflect the ball away from the barrier.
            let r = (ball_direction
                - (2.0 * (ball_direction.dot(axis) * axis)))
                .normalize();
            commands.entity(entity).insert(Movement {
                direction: r,
                speed: movement.speed,
                max_speed: movement.max_speed,
                acceleration: movement.acceleration,
                delta: movement.delta,
            });
            break;
        }

        // Paddle collisions
        for (transform, goal) in paddles_query.iter() {
            let axis = goal.axis();
            let ball_distance = goal.distance_to_ball(ball_transform);
            let ball_position =
                goal.map_ball_position_to_paddle_range(ball_transform);
            let ball_to_paddle = transform.translation.x - ball_position;
            let distance_from_paddle_center = (ball_to_paddle).abs();

            // Check that the ball is touching the paddle and facing the goal.
            if ball_distance > PADDLE_HALF_DEPTH
                || distance_from_paddle_center >= PADDLE_HALF_WIDTH
                || ball_direction.dot(axis) <= 0.0
            {
                continue;
            }

            // Reverse the ball's direction and rotate it outward based on how
            // far its position is from the paddle's center.
            let rotation_away_from_center = Quat::from_rotation_y(
                std::f32::consts::FRAC_PI_4
                    * (ball_to_paddle / PADDLE_HALF_WIDTH),
            );
            let r = rotation_away_from_center * -ball_direction;

            commands.entity(entity).insert(Movement {
                direction: r,
                speed: movement.speed,
                max_speed: movement.max_speed,
                acceleration: movement.acceleration,
                delta: movement.delta,
            });
            break;
        }

        // Barrier collisions
        for barrier_transform in barriers_query.iter() {
            let ball_to_barrier_distance = ball_transform
                .translation
                .distance(barrier_transform.translation);

            // Prevent balls from deflecting through the floor.
            let mut axis =
                barrier_transform.translation - ball_transform.translation;

            axis.y = 0.0;
            axis = axis.normalize();

            // Check that the ball is touching the barrier and facing it.
            if ball_to_barrier_distance > BARRIER_RADIUS + BALL_RADIUS
                || ball_direction.dot(axis) <= 0.0
            {
                continue;
            }

            // Deflect the ball away from the barrier.
            let r = (ball_direction
                - (2.0 * (ball_direction.dot(axis) * axis)))
                .normalize();
            commands.entity(entity).insert(Movement {
                direction: r,
                speed: movement.speed,
                max_speed: movement.max_speed,
                acceleration: movement.acceleration,
                delta: movement.delta,
            });
            break;
        }

        // Wall collisions
        for goal in walls_query.iter() {
            let ball_distance = goal.distance_to_ball(ball_transform);
            let axis = goal.axis();

            // Check that the ball is touching the wall and facing the goal.
            if ball_distance > WALL_RADIUS || ball_direction.dot(axis) <= 0.0 {
                continue;
            }

            // Deflect the ball away from the wall.
            let r = (ball_direction
                - (2.0 * (ball_direction.dot(axis) * axis)))
                .normalize();
            commands.entity(entity).insert(Movement {
                direction: r,
                speed: movement.speed,
                max_speed: movement.max_speed,
                acceleration: movement.acceleration,
                delta: movement.delta,
            });
            break;
        }
    }
}

/// Checks if a `Ball` has scored against a `Goal` and then decrements the
/// corresponding score.
pub fn scored_system(
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut goal_eliminated_writer: EventWriter<GoalEliminated>,
    balls_query: Query<
        (Entity, &GlobalTransform),
        (With<Ball>, With<Active>, Without<Fade>),
    >,
    walls_query: Query<&Goal, With<Wall>>,
) {
    for (entity, ball_transform) in balls_query.iter() {
        for goal in walls_query.iter() {
            // Score against the goal that's closest to this ball.
            let ball_distance = goal.distance_to_ball(ball_transform);

            if ball_distance > 0.0 {
                continue;
            }

            // Decrement the score and potentially eliminate the goal.
            let score = game.scores.get_mut(goal).unwrap();
            *score = score.saturating_sub(1);

            if *score == 0 {
                goal_eliminated_writer.send(GoalEliminated(*goal))
            }

            // Fade out and deactivate the ball to prevent repeated scoring.
            commands.entity(entity).insert(Fade::Out(0.0));
            break;
        }
    }
}
