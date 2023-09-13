use bevy::prelude::*;

use crate::{
    ball::Ball,
    collider::Collider,
    crab::{
        AiPlayer, Crab, Target, CRAB_CENTER_HIT_AREA_PERCENTAGE, CRAB_WIDTH,
    },
    movement::{Heading, Movement, StoppingDistance},
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
                display_crab_predicted_stop_position_gizmos,
                display_crab_to_ball_targeting_gizmos,
                display_ai_crab_ideal_hit_area_gizmos,
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
        (With<Ball>, With<Movement>),
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

fn display_crab_predicted_stop_position_gizmos(
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

fn display_crab_to_ball_targeting_gizmos(
    crabs_query: Query<
        (&GlobalTransform, &Target),
        (With<AiPlayer>, With<Crab>, With<Movement>),
    >,
    balls_query: Query<
        &GlobalTransform,
        (With<Ball>, With<Movement>, With<Collider>),
    >,
    mut gizmos: Gizmos,
) {
    for (crab_transform, target) in &crabs_query {
        if let Ok(ball_transform) = balls_query.get(target.0) {
            gizmos.line(
                crab_transform.translation(),
                ball_transform.translation(),
                Color::PURPLE,
            );
        }
    }
}

fn display_ai_crab_ideal_hit_area_gizmos(
    crabs_query: Query<
        &GlobalTransform,
        (With<Crab>, With<AiPlayer>, With<Movement>),
    >,
    mut gizmos: Gizmos,
) {
    for global_transform in &crabs_query {
        let mut hit_area_transform = global_transform.compute_transform();

        hit_area_transform.scale.x =
            CRAB_CENTER_HIT_AREA_PERCENTAGE * CRAB_WIDTH;
        gizmos.cuboid(hit_area_transform, Color::YELLOW);
    }
}

// TODO: Add debug visualizations for bounding shapes?
