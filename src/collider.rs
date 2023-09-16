use bevy::prelude::*;

use crate::{
    level::{
        barrier::{Barrier, BARRIER_RADIUS},
        side::Side,
    },
    movement::{Heading, Movement},
    object::{
        ball::{Ball, BALL_RADIUS},
        crab::{Crab, CRAB_DEPTH, CRAB_WIDTH},
        wall::{Wall, WALL_RADIUS},
    },
    state::GameState,
};

/// Marks a collidable entity.
#[derive(Component, Debug)]
pub struct Collider;

/// For systems that handle collisions that modify a [`Ball`] [`Heading`].
#[derive(SystemSet, Clone, Hash, Debug, PartialEq, Eq)]
pub struct ColliderSet;

pub struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            PostUpdate,
            ColliderSet.run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            PostUpdate,
            (
                ball_and_ball_collisions,
                barrier_and_ball_collisions,
                crab_and_ball_collisions,
                wall_and_ball_collisions,
            )
                .in_set(ColliderSet),
        );
    }
}

/// Get a deflection direction based on a ball's delta from a crab's center.
pub fn calculate_ball_to_paddle_deflection(delta: f32, axis: Vec3) -> Vec3 {
    let rotation_away_from_center = Quat::from_rotation_y(
        std::f32::consts::FRAC_PI_4
            * (delta / (0.5 * CRAB_WIDTH)).clamp(-1.0, 1.0),
    );
    let deflection_direction = rotation_away_from_center * -axis;

    deflection_direction
}

fn reflect(d: Vec3, n: Vec3) -> Vec3 {
    (d - (2.0 * (d.dot(n) * n))).normalize()
}

fn ball_and_ball_collisions(
    mut commands: Commands,
    balls_query: Query<
        (Entity, &GlobalTransform, &Heading),
        (With<Ball>, With<Movement>, With<Collider>),
    >,
) {
    for [(entity1, transform1, heading1), (entity2, transform2, heading2)] in
        balls_query.iter_combinations()
    {
        // Check that both balls are close enough to touch.
        let delta = transform2.translation() - transform1.translation();

        if delta.length() > BALL_RADIUS + BALL_RADIUS {
            continue;
        }

        // Deflect both balls away from each other.
        let axis1 = delta.normalize();
        let axis2 = -axis1;
        let is_b1_facing_b2 = heading1.0.dot(axis1) > 0.0;
        let is_b2_facing_b1 = heading2.0.dot(axis2) > 0.0;

        if is_b1_facing_b2 {
            commands
                .entity(entity1)
                .insert(Heading(reflect(heading1.0, axis1)));
        } else if is_b2_facing_b1 {
            commands.entity(entity1).insert(Heading(axis2));
        }

        if is_b2_facing_b1 {
            commands
                .entity(entity2)
                .insert(Heading(reflect(heading2.0, axis2)));
        } else if is_b1_facing_b2 {
            commands.entity(entity2).insert(Heading(axis1));
        }

        info!("Ball({:?}): Collided Ball({:?})", entity1, entity2);
    }
}

fn barrier_and_ball_collisions(
    mut commands: Commands,
    balls_query: Query<
        (Entity, &GlobalTransform, &Heading),
        (With<Ball>, With<Collider>, With<Movement>),
    >,
    barriers_query: Query<
        (Entity, &GlobalTransform),
        (With<Barrier>, With<Collider>),
    >,
) {
    for (ball_entity, ball_transform, ball_heading) in &balls_query {
        for (barrier_entity, barrier_transform) in &barriers_query {
            // Prevent balls from deflecting through the floor.
            let delta =
                barrier_transform.translation() - ball_transform.translation();
            let mut axis = delta;

            axis.y = 0.0;
            axis = axis.normalize();

            // Check that the ball is touching the barrier and facing it.
            if delta.length() > BARRIER_RADIUS + BALL_RADIUS
                || ball_heading.0.dot(axis) <= 0.0
            {
                continue;
            }

            // Deflect the ball away from the barrier.
            commands
                .entity(ball_entity)
                .insert(Heading(reflect(ball_heading.0, axis)));

            info!(
                "Ball({:?}): Collided Barrier({:?})",
                ball_entity, barrier_entity
            );
            break;
        }
    }
}

fn crab_and_ball_collisions(
    mut commands: Commands,
    balls_query: Query<
        (Entity, &GlobalTransform, &Heading),
        (With<Ball>, With<Collider>, With<Movement>),
    >,
    crabs_query: Query<(&Side, &Transform), (With<Crab>, With<Collider>)>,
) {
    for (ball_entity, ball_transform, ball_heading) in &balls_query {
        for (side, crab_transform) in &crabs_query {
            // Check that the ball is touching the crab and facing the goal.
            let axis = side.axis();
            let ball_to_goal_distance = side.distance_to_ball(ball_transform);
            let ball_goal_position = side.get_ball_position(ball_transform);
            let delta = crab_transform.translation.x - ball_goal_position;
            let ball_to_crab_distance = delta.abs();

            if ball_to_goal_distance > BALL_RADIUS + (0.5 * CRAB_DEPTH)
                || ball_to_crab_distance > BALL_RADIUS + (0.5 * CRAB_WIDTH)
                || ball_heading.0.dot(axis) <= 0.0
            {
                continue;
            }

            let ball_deflection_direction =
                calculate_ball_to_paddle_deflection(delta, axis);

            commands
                .entity(ball_entity)
                .insert(Heading(ball_deflection_direction));
            info!("Ball({:?}): Collided Crab({:?})", ball_entity, side);
            break;
        }
    }
}

fn wall_and_ball_collisions(
    mut commands: Commands,
    balls_query: Query<
        (Entity, &GlobalTransform, &Heading),
        (With<Ball>, With<Collider>, With<Movement>),
    >,
    walls_query: Query<&Side, (With<Wall>, With<Collider>)>,
) {
    for (entity, ball_transform, ball_heading) in &balls_query {
        for side in &walls_query {
            let ball_to_wall_distance = side.distance_to_ball(ball_transform);
            let axis = side.axis();

            // Check that the ball is touching and facing the wall.
            if ball_to_wall_distance > BALL_RADIUS + WALL_RADIUS
                || ball_heading.0.dot(axis) <= 0.0
            {
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

    // TODO: Need a fix for the rare occasion when a ball just bounces infinitely
    // between two walls in a straight line? Maybe make all bounces slightly adjust
    // ball angle rather than pure reflection?
}
