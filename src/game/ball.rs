use super::*;
use crate::GameConfig;
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
        (&mut Fade, &mut Visible, &Handle<StandardMaterial>),
        With<Ball>,
    >,
) {
    let mut is_prior_resetting = false;

    for (mut fade, mut visible, material) in query.iter_mut() {
        let is_current_resetting = matches!(*fade, Fade::In(_));

        // Force current ball to wait if other is also fading in
        if is_prior_resetting && is_current_resetting {
            *fade = Fade::In(0.0);
            continue;
        }

        is_prior_resetting = is_current_resetting;

        // Alpha-blend the balls only when necessary.
        visible.is_transparent = fade.opacity() < 1.0;
        materials
            .get_mut(material)
            .unwrap()
            .base_color
            .set_a(fade.opacity());
    }
}

/// Takes a fully hidden `Ball`, disables its movement, moves it back to the
/// center of the arena, and then slowly fades it back into view.
pub fn inactive_ball_reset_system(
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut Transform, &mut Movement),
        (With<Ball>, Without<Fade>, Without<Active>),
    >,
) {
    for (entity, mut transform, mut movement) in query.iter_mut() {
        transform.translation = BALL_CENTER_POINT;
        movement.dead_stop();
        commands.entity(entity).insert(Fade::In(0.0));
        info!("Ball({:?}) -> Resetting", entity);
    }
}

/// Takes a newly `Active` `Ball` and gives it `Movement` so that it starts it
/// moving in a straight line in a random direction.
pub fn reactivated_ball_launch_system(
    config: Res<GameConfig>,
    mut query: Query<
        (Entity, &mut Movement),
        (With<Ball>, Without<Fade>, Added<Active>),
    >,
) {
    for (entity, mut movement) in query.iter_mut() {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);

        movement.direction = Vec3::new(angle.cos(), 0.0, angle.sin());
        movement.speed = config.ball_starting_speed;
        movement.delta = Some(Delta::Positive);
        info!("Ball({:?}) -> Launched", entity);
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

        // Ball collisions
        for (entity2, transform2, _) in balls_query.iter() {
            // Prevent balls from colliding with themselves.
            if entity == entity2 {
                continue;
            }

            let ball_to_ball_distance =
                ball_transform.translation.distance(transform2.translation);
            let axis = (transform2.translation - ball_transform.translation)
                .normalize();

            // Check that the ball is touching the other ball and facing it.
            if ball_to_ball_distance > BALL_RADIUS + BALL_RADIUS
                || ball_direction.dot(axis) <= 0.0
            {
                continue;
            }

            // Deflect the ball away from the other ball.
            let r = reflect(ball_direction, axis);
            commands.entity(entity).insert(Movement {
                direction: r,
                speed: movement.speed,
                max_speed: movement.max_speed,
                acceleration: movement.acceleration,
                delta: movement.delta,
            });
            info!("Ball({:?}) -> Collided Ball({:?})", entity, entity2);
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
            info!("Ball({:?}) -> Collided Paddle({:?})", entity, goal);
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
            let r = reflect(ball_direction, axis);
            commands.entity(entity).insert(Movement {
                direction: r,
                speed: movement.speed,
                max_speed: movement.max_speed,
                acceleration: movement.acceleration,
                delta: movement.delta,
            });
            info!("Ball({:?}) -> Collided Barrier", entity);
            break;
        }

        // Wall collisions
        for goal in walls_query.iter() {
            let ball_distance = goal.distance_to_ball(ball_transform);
            let axis = goal.axis();

            // Check that the ball is touching and facing the wall.
            if ball_distance > WALL_RADIUS || ball_direction.dot(axis) <= 0.0 {
                continue;
            }

            // Deflect the ball away from the wall.
            let r = reflect(ball_direction, axis);
            commands.entity(entity).insert(Movement {
                direction: r,
                speed: movement.speed,
                max_speed: movement.max_speed,
                acceleration: movement.acceleration,
                delta: movement.delta,
            });
            info!("Ball({:?}) -> Collided Wall({:?})", entity, goal);
            break;
        }
    }
}

/// Checks if a `Ball` has scored against a `Goal` and then decrements the
/// corresponding score.
pub fn goal_scored_system(
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
            // A ball will score against the goal it's closest to once it is
            // fully past said goal's paddle.
            let ball_distance = goal.distance_to_ball(ball_transform);

            if ball_distance > -PADDLE_HALF_DEPTH {
                continue;
            }

            // Decrement the score and potentially eliminate the goal.
            let score = game.scores.get_mut(goal).unwrap();

            *score = score.saturating_sub(1);
            info!("Ball({:?}) -> Scored {:?}", entity, goal);

            if *score == 0 {
                goal_eliminated_writer.send(GoalEliminated(*goal));
                info!("Ball({:?}) -> Eliminated {:?}", entity, goal);
            }

            // Fade out and deactivate the ball to prevent repeated scoring.
            commands.entity(entity).insert(Fade::Out(0.0));
            break;
        }
    }
}

/// A basic reflect function that also normalizes the result.
fn reflect(d: Vec3, n: Vec3) -> Vec3 {
    (d - (2.0 * (d.dot(n) * n))).normalize()
}
