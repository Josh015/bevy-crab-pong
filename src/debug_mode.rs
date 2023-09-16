use bevy::prelude::*;

use crate::{
    ball::Ball,
    collider::{calculate_ball_to_paddle_deflection, Collider, ColliderSet},
    crab::{Crab, CRAB_WIDTH},
    movement::{Heading, Movement, StoppingDistance},
    player::ai::{PlayerAi, Target, AI_CENTER_HIT_AREA_PERCENTAGE},
    side::Side,
};

pub const DEBUGGING_RAY_LENGTH: f32 = 20.0;

/// Toggles displaying debugging gizmos.
#[derive(Debug, Default, Resource)]
pub struct IsDebuggingMode(pub bool);

pub struct DebugModePlugin;

impl Plugin for DebugModePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<IsDebuggingMode>().add_systems(
            PostUpdate,
            (
                ball_movement_direction_gizmos,
                crab_stop_position_gizmos,
                crab_collider_ball_deflection_direction_gizmos,
                crab_ai_ball_targeting_gizmos,
                crab_ai_ideal_ball_hit_area_gizmos,
            )
                .after(ColliderSet)
                .run_if(
                    |is_debugging_mode: Res<IsDebuggingMode>| {
                        is_debugging_mode.0
                    },
                ),
        );
    }
}

fn ball_movement_direction_gizmos(
    balls_query: Query<
        (&GlobalTransform, &Heading),
        (With<Ball>, With<Movement>),
    >,
    mut gizmos: Gizmos,
) {
    for (global_transform, heading) in &balls_query {
        gizmos.line(
            global_transform.translation(),
            global_transform.translation() + heading.0 * DEBUGGING_RAY_LENGTH,
            Color::RED,
        )
    }
}

fn crab_stop_position_gizmos(
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

fn crab_collider_ball_deflection_direction_gizmos(
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
                calculate_ball_to_paddle_deflection(ball_to_crab, axis);

            gizmos.line(
                crab_global_transform.translation(),
                crab_global_transform.translation()
                    + DEBUGGING_RAY_LENGTH * ball_deflection_direction,
                Color::WHITE,
            );
        }
    }
}

fn crab_ai_ball_targeting_gizmos(
    crabs_query: Query<
        (&GlobalTransform, &Target),
        (With<PlayerAi>, With<Crab>, With<Movement>),
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

fn crab_ai_ideal_ball_hit_area_gizmos(
    crabs_query: Query<
        &GlobalTransform,
        (With<PlayerAi>, With<Crab>, With<Movement>),
    >,
    mut gizmos: Gizmos,
) {
    for global_transform in &crabs_query {
        let mut hit_area_transform = global_transform.compute_transform();

        hit_area_transform.scale.x = AI_CENTER_HIT_AREA_PERCENTAGE * CRAB_WIDTH;
        gizmos.cuboid(hit_area_transform, Color::YELLOW);
    }
}

// TODO: Add debug visualizations for bounding shapes?
