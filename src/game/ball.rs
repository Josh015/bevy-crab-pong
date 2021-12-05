use bevy::{ecs::prelude::*, prelude::*};
use rand::prelude::*;

use super::{
    barrier::Barrier,
    fade::{Active, Fade},
    goal::Goal,
    paddle::Paddle,
    velocity::Velocity,
    wall::Wall,
    Delta, Game, GoalEliminated,
};
use crate::GameConfig;

#[derive(Component)]
pub struct Ball;

pub fn step_fade_animation_system(
    config: Res<GameConfig>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<
        (&Handle<StandardMaterial>, &mut Transform, &mut Fade),
        With<Ball>,
    >,
) {
    // Increase/Decrease balls' opacity to show/hide them
    let mut is_prior_fading = false;

    for (material, mut transform, mut fade) in query.iter_mut() {
        let is_current_fading = matches!(*fade, Fade::In(_));

        // Force current ball to wait if other is also fading in
        if is_prior_fading && is_current_fading {
            *fade = Fade::In(0.0);
            continue;
        }

        is_prior_fading = is_current_fading;

        // materials
        //     .get_mut(material)
        //     .unwrap()
        //     .base_color
        //     .set_a(fade.opacity());

        // FIXME: Use scaling until we can get alpha-blending working
        transform.scale = Vec3::splat(fade.opacity() * config.ball_size);
    }
}

pub fn reset_position_system(
    mut commands: Commands,
    config: Res<GameConfig>,
    mut query: Query<
        (Entity, &mut Transform),
        (With<Ball>, Without<Fade>, Without<Active>),
    >,
) {
    for (entity, mut transform) in query.iter_mut() {
        transform.translation = config.ball_center_point();
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
            speed: 0.0,
            max_speed: config.ball_speed,
            acceleration: 1.5,
            delta: Delta::Accelerating(1.0),
        });
    }
}

pub fn collision_system(
    mut commands: Commands,
    config: Res<GameConfig>,
    balls_query: Query<
        (Entity, &GlobalTransform, &Velocity),
        (With<Ball>, With<Active>),
    >,
    paddles_query: Query<&GlobalTransform, (With<Paddle>, With<Active>)>,
    barriers_query: Query<&GlobalTransform, With<Barrier>>,
    walls_query: Query<(&GlobalTransform, &Goal), (With<Wall>, With<Active>)>,
) {
    for (entity, transform, velocity) in balls_query.iter() {
        let ball_radius = config.ball_radius();
        let barrier_radius = 0.5 * config.barrier_width;
        let half_width = 0.5 * config.beach_width;
        let d = velocity.direction;
        let radius_position = transform.translation + d * ball_radius;

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
        for (wall_transform, goal) in walls_query.iter() {
            let wall_position = wall_transform.translation;
            let (n, distance_to_goal) = match *goal {
                Goal::Top => (-Vec3::Z, radius_position.z - wall_position.z),
                Goal::Right => (Vec3::X, -radius_position.x + wall_position.x),
                Goal::Bottom => (Vec3::Z, -radius_position.z + wall_position.z),
                Goal::Left => (-Vec3::X, radius_position.x - wall_position.x),
            };

            // Slight offset from the wall so the ball doesn't phase into it.
            // Also prevents it from being treated as out of bounds.
            if distance_to_goal > 0.01 {
                continue;
            }

            let r = (d - (2.0 * (d.dot(n) * n))).normalize();
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
    config: Res<GameConfig>,
    mut game: ResMut<Game>,
    mut goal_eliminated_writer: EventWriter<GoalEliminated>,
    balls_query: Query<
        (Entity, &GlobalTransform, &Velocity),
        (With<Ball>, With<Active>, Without<Fade>),
    >,
    walls_query: Query<(&GlobalTransform, &Goal), With<Wall>>,
) {
    for (entity, ball_transform, velocity) in balls_query.iter() {
        let ball_translation = ball_transform.translation;
        let ball_radius = config.ball_radius();
        let d = velocity.direction;
        let radius_position = ball_translation + d * ball_radius;

        for (wall_transform, goal) in walls_query.iter() {
            // Score against the goal that's closest to this ball
            let goal_position = wall_transform.translation;
            let distance_to_goal = match *goal {
                Goal::Top => radius_position.z - goal_position.z,
                Goal::Right => -radius_position.x + goal_position.x,
                Goal::Bottom => -radius_position.z + goal_position.z,
                Goal::Left => radius_position.x - goal_position.x,
            };

            if distance_to_goal > 0.0 {
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
