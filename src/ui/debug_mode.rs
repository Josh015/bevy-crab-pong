use bevy::prelude::*;

use crate::{
    common::{
        collider::Collider,
        movement::{Heading, Movement, StoppingDistance},
    },
    game::state::LoadedSet,
    level::side::Side,
    object::{
        ball::Ball,
        crab::{
            ai::{CrabAi, Target, AI_CENTER_HIT_AREA_PERCENTAGE},
            Crab, CRAB_WIDTH,
        },
    },
    util::hemisphere_deflection,
};

pub const DEBUGGING_RAY_LENGTH: f32 = 20.0;

/// Toggles displaying various debugging gizmos.
#[derive(Debug, Default, Resource)]
pub struct DebugMode {
    pub has_ball_movement: bool,
    pub has_crab_stop_positions: bool,
    pub has_crab_ai_ball_targeting: bool,
    pub has_crab_ai_ideal_ball_hit_area: bool,
    pub has_crab_collider_ball_deflection_direction: bool,
}

pub(super) struct DebugModePlugin;

impl Plugin for DebugModePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugMode>()
            .add_systems(
                Update,
                handle_debug_mode_keyboard_toggles.in_set(LoadedSet),
            )
            .add_systems(
                PostUpdate,
                (
                    ball_movement_direction_gizmos.run_if(
                        |debug_mode: Res<DebugMode>| {
                            debug_mode.has_ball_movement
                        },
                    ),
                    crab_stop_position_gizmos.run_if(
                        |debug_mode: Res<DebugMode>| {
                            debug_mode.has_crab_stop_positions
                        },
                    ),
                    crab_ai_ball_targeting_gizmos.run_if(
                        |debug_mode: Res<DebugMode>| {
                            debug_mode.has_crab_ai_ball_targeting
                        },
                    ),
                    crab_ai_ideal_ball_hit_area_gizmos.run_if(
                        |debug_mode: Res<DebugMode>| {
                            debug_mode.has_crab_ai_ideal_ball_hit_area
                        },
                    ),
                    crab_collider_ball_deflection_direction_gizmos.run_if(
                        |debug_mode: Res<DebugMode>| {
                            debug_mode
                                .has_crab_collider_ball_deflection_direction
                        },
                    ),
                )
                    .after(LoadedSet),
            );
    }
}

fn handle_debug_mode_keyboard_toggles(
    keyboard_input: Res<Input<KeyCode>>,
    mut debug_mode: ResMut<DebugMode>,
) {
    if keyboard_input.just_pressed(KeyCode::Key1) {
        let toggle = !debug_mode.has_ball_movement;

        debug_mode.has_ball_movement = toggle;
        debug_mode.has_crab_stop_positions = toggle;
        debug_mode.has_crab_ai_ball_targeting = toggle;
        debug_mode.has_crab_ai_ideal_ball_hit_area = toggle;
        debug_mode.has_crab_collider_ball_deflection_direction = toggle;
    } else if keyboard_input.just_pressed(KeyCode::Key2) {
        debug_mode.has_ball_movement = !debug_mode.has_ball_movement;
    } else if keyboard_input.just_pressed(KeyCode::Key3) {
        debug_mode.has_crab_stop_positions =
            !debug_mode.has_crab_stop_positions;
    } else if keyboard_input.just_pressed(KeyCode::Key4) {
        debug_mode.has_crab_ai_ball_targeting =
            !debug_mode.has_crab_ai_ball_targeting;
    } else if keyboard_input.just_pressed(KeyCode::Key5) {
        debug_mode.has_crab_ai_ideal_ball_hit_area =
            !debug_mode.has_crab_ai_ideal_ball_hit_area;
    } else if keyboard_input.just_pressed(KeyCode::Key6) {
        debug_mode.has_crab_collider_ball_deflection_direction =
            !debug_mode.has_crab_collider_ball_deflection_direction;
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

fn crab_ai_ball_targeting_gizmos(
    crabs_query: Query<
        (&GlobalTransform, &Target),
        (With<CrabAi>, With<Crab>, With<Movement>),
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
        (With<CrabAi>, With<Crab>, With<Movement>),
    >,
    mut gizmos: Gizmos,
) {
    for global_transform in &crabs_query {
        let mut hit_area_transform = global_transform.compute_transform();

        hit_area_transform.scale.x = AI_CENTER_HIT_AREA_PERCENTAGE * CRAB_WIDTH;
        gizmos.cuboid(hit_area_transform, Color::YELLOW);
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
            // Check that the ball is near the crab and facing the side.
            let axis = side.axis();
            let ball_side_position = side.get_ball_position(ball_transform);
            let delta = transform.translation.x - ball_side_position;
            let crab_to_ball_distance = ball_transform
                .translation()
                .distance(crab_global_transform.translation());

            if crab_to_ball_distance > 0.25 || ball_heading.0.dot(axis) <= 0.0 {
                continue;
            }

            let ball_deflection_direction =
                hemisphere_deflection(delta, CRAB_WIDTH, axis);

            gizmos.line(
                crab_global_transform.translation(),
                crab_global_transform.translation()
                    + DEBUGGING_RAY_LENGTH * ball_deflection_direction,
                Color::WHITE,
            );
        }
    }
}

// TODO: Add debug visualizations for bounding shapes?
