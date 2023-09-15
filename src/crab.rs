use bevy::prelude::*;

use crate::{
    ball::{Ball, BALL_RADIUS},
    barrier::BARRIER_RADIUS,
    collider::{Collider, ColliderSet},
    debug_mode::{DebugModeSet, DEBUGGING_RAY_LENGTH},
    goal::GOAL_WIDTH,
    movement::{
        Force, Heading, Movement, MovementSet, Speed, StoppingDistance,
    },
    side::Side,
};

pub const CRAB_WIDTH: f32 = 0.2;
pub const CRAB_DEPTH: f32 = 0.1;
pub const CRAB_START_POSITION: Vec3 = Vec3::new(0.0, 0.05, 0.0);
pub const CRAB_POSITION_X_MAX: f32 =
    (0.5 * GOAL_WIDTH) - BARRIER_RADIUS - (0.5 * CRAB_WIDTH);

/// Makes a crab entity that can deflect balls and move sideways inside a goal.
#[derive(Component, Debug)]
pub struct Crab;

pub struct CrabPlugin;

impl Plugin for CrabPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            restrict_crab_movement_range.after(MovementSet),
        )
        .add_systems(
            PostUpdate,
            (
                crab_and_ball_collisions.in_set(ColliderSet),
                (
                    display_predicted_stop_position_gizmos,
                    display_predicted_ball_deflection_direction_gizmos,
                )
                    .in_set(DebugModeSet),
            ),
        );
    }
}

fn restrict_crab_movement_range(
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut Transform, &mut Speed, &mut StoppingDistance),
        (With<Crab>, With<Movement>),
    >,
) {
    for (entity, mut transform, mut speed, mut stopping_distance) in &mut query
    {
        // Limit crab to bounds of the goal.
        if !(-CRAB_POSITION_X_MAX..=CRAB_POSITION_X_MAX)
            .contains(&transform.translation.x)
        {
            transform.translation.x = transform
                .translation
                .x
                .clamp(-CRAB_POSITION_X_MAX, CRAB_POSITION_X_MAX);
            speed.0 = 0.0;
            commands.entity(entity).remove::<Force>();
        }

        // Limit stopping distance to the bounds of the goal.
        let stopped_position = transform.translation.x + stopping_distance.0;

        if !(-CRAB_POSITION_X_MAX..=CRAB_POSITION_X_MAX)
            .contains(&stopped_position)
        {
            stopping_distance.0 = stopped_position.signum()
                * CRAB_POSITION_X_MAX
                - transform.translation.x;
        }
    }
}

fn crab_and_ball_collisions(
    mut commands: Commands,
    balls_query: Query<
        (Entity, &GlobalTransform, &Heading),
        (With<Ball>, With<Movement>, With<Collider>),
    >,
    crabs_query: Query<(&Side, &Transform), (With<Crab>, With<Collider>)>,
) {
    for (ball_entity, ball_transform, ball_heading) in &balls_query {
        for (side, crab_transform) in &crabs_query {
            // Check that the ball is touching the crab and facing the goal.
            let axis = side.axis();
            let ball_distance_to_goal = side.distance_to_ball(ball_transform);
            let ball_goal_position = side.get_ball_position(ball_transform);
            let ball_to_crab =
                crab_transform.translation.x - ball_goal_position;
            let ball_distance_to_crab = ball_to_crab.abs();

            if ball_distance_to_goal > BALL_RADIUS + (0.5 * CRAB_DEPTH)
                || ball_distance_to_crab > BALL_RADIUS + (0.5 * CRAB_WIDTH)
                || ball_heading.0.dot(axis) <= 0.0
            {
                continue;
            }

            let ball_deflection_direction =
                calculate_ball_deflection_direction(ball_to_crab, axis);

            commands
                .entity(ball_entity)
                .insert(Heading(ball_deflection_direction));
            info!("Ball({:?}): Collided Crab({:?})", ball_entity, side);
            break;
        }
    }
}

fn display_predicted_stop_position_gizmos(
    crabs_query: Query<
        (&GlobalTransform, &Heading, &StoppingDistance),
        (With<Crab>, With<Movement>),
    >,
    mut gizmos: Gizmos,
) {
    for (global_transform, heading, stopping_distance) in &crabs_query {
        let mut stop_position_transform = global_transform.compute_transform();
        let global_heading = stop_position_transform.rotation * heading.0;

        stop_position_transform.translation +=
            global_heading * stopping_distance.0;
        gizmos.line(
            global_transform.translation(),
            stop_position_transform.translation,
            Color::BLUE,
        );
        gizmos.cuboid(stop_position_transform, Color::GREEN);
    }
}

fn display_predicted_ball_deflection_direction_gizmos(
    balls_query: Query<
        (&GlobalTransform, &Heading),
        (With<Ball>, With<Movement>, With<Collider>),
    >,
    crabs_query: Query<
        (&Side, &Transform, &GlobalTransform),
        (With<Crab>, With<Collider>),
    >,
    mut gizmos: Gizmos,
) {
    for (ball_transform, ball_heading) in &balls_query {
        for (side, transform, crab_global_transform) in &crabs_query {
            // Check that the ball is near the crab and facing the goal.
            let axis = side.axis();
            let ball_goal_position = side.get_ball_position(ball_transform);
            let ball_to_crab = transform.translation.x - ball_goal_position;
            let ball_to_crab_distance = ball_transform
                .translation()
                .distance(crab_global_transform.translation());

            if ball_to_crab_distance > 0.25 || ball_heading.0.dot(axis) <= 0.0 {
                continue;
            }

            let ball_deflection_direction =
                calculate_ball_deflection_direction(ball_to_crab, axis);

            gizmos.line(
                crab_global_transform.translation(),
                crab_global_transform.translation()
                    + DEBUGGING_RAY_LENGTH * ball_deflection_direction,
                Color::WHITE,
            );
        }
    }
}

fn calculate_ball_deflection_direction(
    ball_to_crab_horizontal_distance: f32,
    paddle_axis: Vec3,
) -> Vec3 {
    // Deflection angle is based on the ball's distance from the center of the
    // crab. This way players can influence where the ball will go.
    let rotation_away_from_center = Quat::from_rotation_y(
        std::f32::consts::FRAC_PI_4
            * (ball_to_crab_horizontal_distance / (0.5 * CRAB_WIDTH))
                .clamp(-1.0, 1.0),
    );

    rotation_away_from_center * -paddle_axis
}
