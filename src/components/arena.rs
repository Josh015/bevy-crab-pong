use bevy::{ecs::prelude::*, prelude::*};
use rand::prelude::*;

use crate::GameConfig;

use super::{
    ball::Ball,
    barrier::Barrier,
    fade::{Active, Fade},
    goal::Goal,
    paddle::Paddle,
    velocity::Velocity,
    wall::Wall,
};

#[derive(Component)]
pub struct Arena;

pub fn reset_ball_position_system(
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

pub fn reset_ball_velocity_system(
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
        let velocity =
            config.ball_speed * Vec3::new(angle.cos(), 0.0, angle.sin());

        commands.entity(entity).insert(Velocity(velocity));
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
        let d = velocity.0.normalize();
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
            let (n, distance_to_goal) = match goal {
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
            commands
                .entity(entity)
                .insert(Velocity(r * config.ball_speed));
            break;
        }
    }
}
