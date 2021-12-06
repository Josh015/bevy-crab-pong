use super::*;
use crate::GameConfig;
use bevy::{ecs::prelude::*, prelude::*};
use rand::prelude::*;

#[derive(Component)]
pub struct Ball;

pub fn fade_animation_system(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<
        (&Handle<StandardMaterial>, &mut Transform, &mut Fade),
        With<Ball>,
    >,
) {
    // Increase/Decrease balls' opacity to show/hide them
    let mut is_prior_resetting = false;

    for (material, mut transform, mut fade) in query.iter_mut() {
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
            .remove::<Velocity>()
            .insert(Fade::In(0.0));
    }
}

pub fn reset_velocity_system(
    mut commands: Commands,
    config: Res<GameConfig>,
    query: Query<Entity, (With<Ball>, Without<Velocity>, Added<Active>)>,
) {
    for entity in query.iter() {
        // TODO: Move this into a global resource? Also, make a reusable uniform
        // for random rotation?
        let mut rng = rand::thread_rng();

        // Give the ball a random direction vector
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);

        commands.entity(entity).insert(Velocity {
            direction: Vec3::new(angle.cos(), 0.0, angle.sin()),
            speed: config.ball_starting_speed,
            max_speed: config.ball_max_speed,
            acceleration: config.ball_max_speed
                / config.ball_seconds_to_max_speed,
            delta: Delta::Accelerating(1.0),
        });
    }
}

pub fn collision_system(
    mut commands: Commands,
    balls_query: Query<
        (Entity, &GlobalTransform, &Velocity),
        (With<Ball>, With<Active>),
    >,
    paddles_query: Query<&GlobalTransform, (With<Paddle>, With<Active>)>,
    barriers_query: Query<&GlobalTransform, With<Barrier>>,
    walls_query: Query<&Goal, (With<Wall>, With<Active>)>,
) {
    for (entity, ball_transform, velocity) in balls_query.iter() {
        let ball_direction = velocity.direction;

        // TODO: Order these so that highest precedence collision type is at the
        // bottom, since it can overwrite others!

        // Ball collisions
        for (entity2, transform2, velocity2) in balls_query.iter() {
            break;
        }

        // Paddle collisions
        for transform in paddles_query.iter() {
            break;
        }

        // Barrier collisions
        for transform in barriers_query.iter() {
            break;
        }

        // Wall collisions
        for goal in walls_query.iter() {
            let ball_distance = goal.distance_to_ball(ball_transform);
            let axis = goal.axis();

            // Check that the ball is touching the wall and facing the goal
            if ball_distance > WALL_RADIUS || ball_direction.dot(axis) <= 0.0 {
                continue;
            }

            // Deflect the ball away from the wall.
            let r = (ball_direction
                - (2.0 * (ball_direction.dot(axis) * axis)))
                .normalize();
            commands.entity(entity).insert(Velocity {
                direction: r,
                speed: velocity.speed,
                max_speed: velocity.max_speed,
                acceleration: velocity.acceleration,
                delta: velocity.delta,
            });
            break;
        }
    }
}

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
            // Score against the goal that's closest to this ball
            let ball_distance = goal.distance_to_ball(ball_transform);

            if ball_distance > 0.0 {
                continue;
            }

            // Decrement the score and potentially eliminate the goal
            let score = game.scores.get_mut(goal).unwrap();
            *score = score.saturating_sub(1);

            if *score == 0 {
                goal_eliminated_writer.send(GoalEliminated(*goal))
            }

            // Fade out and deactivate the ball to prevent repeated scoring
            commands.entity(entity).insert(Fade::Out(0.0));
            break;
        }
    }
}
