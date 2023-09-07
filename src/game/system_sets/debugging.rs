#![allow(clippy::type_complexity)]

use crate::prelude::*;

// TODO: Make this work with all object movement, not just Balls?
fn debug_ball_paths(
    query: Query<(&GlobalTransform, &Heading), (With<Ball>, Without<Fade>)>,
    mut gizmos: Gizmos,
) {
    for (global_transform, heading) in &query {
        gizmos.line(
            global_transform.translation(),
            global_transform.translation() + heading.0 * 20.0,
            Color::RED,
        )
    }
}

/// Visualizes where the paddles will be when they stop.
fn debug_paddle_stop_positions(
    query: Query<
        (&GlobalTransform, &Heading, &StoppingDistance),
        Without<Fade>,
    >,
    mut gizmos: Gizmos,
) {
    for (global_transform, heading, stopping_distance) in &query {
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

/// Provides debug visualization to show which [`Ai`] entities are targeting
/// which [`Ball`] entities.
fn debug_targeting(
    paddles_query: Query<
        (&GlobalTransform, &Target),
        (With<AiInput>, With<Paddle>, Without<Fade>),
    >,
    balls_query: Query<&GlobalTransform, (With<Ball>, With<Collider>)>,
    mut gizmos: Gizmos,
) {
    for (paddle_transform, target) in &paddles_query {
        if let Ok(ball_transform) = balls_query.get(target.0) {
            gizmos.line(
                paddle_transform.translation(),
                ball_transform.translation(),
                Color::PURPLE,
            );
        }
    }
}

/// Provides debug visualization to show the size of the ideal hit area on each
/// [`Ai`] [`Paddle`] entity.
fn debug_ai_paddle_hit_area(
    paddles_query: Query<
        &GlobalTransform,
        (With<Paddle>, With<AiInput>, Without<Fade>),
    >,
    mut gizmos: Gizmos,
) {
    for global_transform in &paddles_query {
        let mut hit_area_transform = global_transform.compute_transform();

        hit_area_transform.scale.x =
            PADDLE_CENTER_HIT_AREA_PERCENTAGE * PADDLE_WIDTH;
        gizmos.cuboid(hit_area_transform, Color::YELLOW);
    }
}

// TODO: Add debug visualizations for bounding shapes.

pub struct DebuggingPlugin;

impl Plugin for DebuggingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                debug_ball_paths,
                debug_paddle_stop_positions,
                debug_targeting,
                debug_ai_paddle_hit_area,
            )
                .in_set(GameSystemSet::Debugging),
        );
    }
}
