#![allow(clippy::type_complexity)]

use crate::prelude::*;

fn show_debug_gizmos(run_state: Res<RunState>) -> bool {
    run_state.has_debug_gizmos
}

fn debug_ball_path_gizmos(
    query: Query<(&GlobalTransform, &Heading), (With<Ball>, Without<Fade>)>,
    mut gizmos: Gizmos,
) {
    for (global_transform, heading) in &query {
        gizmos.line(
            global_transform.translation(),
            global_transform.translation() + heading.0 * 20.0,
            Color::RED,
        )
        // TODO: Draw a sphere over the goal position where the ball is expected
        // to cross.
    }
}

fn debug_paddle_stop_positions_gizmos(
    query: Query<
        (&GlobalTransform, &Heading, &Acceleration, &Speed),
        (With<Paddle>, Without<Fade>),
    >,
    mut gizmos: Gizmos,
) {
    for (global_transform, heading, acceleration, speed) in &query {
        const DELTA_SECONDS: f32 = 0.01;
        let delta_speed = acceleration.0 * DELTA_SECONDS;
        let mut current_speed = speed.0;
        let mut stop_position_transform = global_transform.compute_transform();
        let global_heading = stop_position_transform.rotation * heading.0;

        // TODO: Need to account for wall collisions.
        while current_speed.abs() > 0.0 {
            stop_position_transform.translation +=
                global_heading * current_speed * DELTA_SECONDS;
            current_speed = decelerate_speed(current_speed, delta_speed);
        }

        gizmos.line(
            global_transform.translation(),
            stop_position_transform.translation,
            Color::BLUE,
        );
        gizmos.cuboid(stop_position_transform, Color::GREEN);
    }
}

fn debug_paddle_target_ball_gizmos(
    query: Query<(&GlobalTransform, &Side), (With<Paddle>, Without<Fade>)>,
    balls_query: Query<&GlobalTransform, (With<Ball>, With<Collider>)>,
    mut gizmos: Gizmos,
) {
    for (global_transform, side) in &query {
        // Get the relative position of the ball that's closest to this goal.
        let mut closest_ball_distance = std::f32::MAX;
        let mut nearest_ball_transform = GlobalTransform::IDENTITY;

        for ball_transform in &balls_query {
            let ball_distance_to_goal = side.distance_to_ball(ball_transform);

            if ball_distance_to_goal >= closest_ball_distance {
                continue;
            }

            closest_ball_distance = ball_distance_to_goal;
            nearest_ball_transform = ball_transform.clone();
        }

        gizmos.line(
            global_transform.translation(),
            nearest_ball_transform.translation(),
            Color::PURPLE,
        );
    }
}

pub struct GizmosPlugin;

impl Plugin for GizmosPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                debug_ball_path_gizmos,
                debug_paddle_stop_positions_gizmos,
                debug_paddle_target_ball_gizmos,
            )
                .run_if(show_debug_gizmos),
        );
    }
}
