use bevy::prelude::*;

use crate::{
    ball::Ball,
    fade::Fade,
    movement::{Heading, StoppingDistance},
    paddle::{
        AiPlayer, Paddle, Target, PADDLE_CENTER_HIT_AREA_PERCENTAGE,
        PADDLE_WIDTH,
    },
};

/// Toggles displaying debugging gizmos.
#[derive(Debug, Default, Resource)]
pub struct IsDebuggingMode(pub bool);

pub struct DebugModePlugin;

impl Plugin for DebugModePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                display_ball_movement_direction_gizmos,
                display_paddle_predicted_stop_position_gizmos,
                display_paddle_to_ball_targeting_gizmos,
                display_ai_paddle_ideal_hit_area_gizmos,
            )
                .run_if(show_debugging_gizmos),
        )
        .init_resource::<IsDebuggingMode>();
    }
}

fn show_debugging_gizmos(is_debugging_mode: Res<IsDebuggingMode>) -> bool {
    is_debugging_mode.0
}

// TODO: Make this work with all object movement, not just Balls?
fn display_ball_movement_direction_gizmos(
    balls_query: Query<
        (&GlobalTransform, &Heading),
        (With<Ball>, Without<Fade>),
    >,
    mut gizmos: Gizmos,
) {
    for (global_transform, heading) in &balls_query {
        gizmos.line(
            global_transform.translation(),
            global_transform.translation() + heading.0 * 20.0,
            Color::RED,
        )
    }
}

fn display_paddle_predicted_stop_position_gizmos(
    paddles_query: Query<
        (&GlobalTransform, &Heading, &StoppingDistance),
        (With<Paddle>, Without<Fade>),
    >,
    mut gizmos: Gizmos,
) {
    for (global_transform, heading, stopping_distance) in &paddles_query {
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

fn display_paddle_to_ball_targeting_gizmos(
    paddles_query: Query<
        (&GlobalTransform, &Target),
        (With<AiPlayer>, With<Paddle>, Without<Fade>),
    >,
    balls_query: Query<&GlobalTransform, (With<Ball>, Without<Fade>)>,
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

fn display_ai_paddle_ideal_hit_area_gizmos(
    paddles_query: Query<
        &GlobalTransform,
        (With<Paddle>, With<AiPlayer>, Without<Fade>),
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

// TODO: Add debug visualizations for bounding shapes?
