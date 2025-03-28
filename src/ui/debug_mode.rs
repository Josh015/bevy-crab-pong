use bevy::prelude::*;

use crate::{
    components::{
        AI, Ball, Collider, Crab, CrabCollider, Direction,
        IDEAL_HIT_AREA_PERCENTAGE, Motion, StoppingDistance, Target,
    },
    system_params::Goals,
    system_sets::ActiveAfterLoadingSet,
};

pub const DEBUGGING_RAY_LENGTH: f32 = 20.0;

pub(super) struct DebugModePlugin;

impl Plugin for DebugModePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugMode>()
            .add_systems(
                Update,
                handle_debug_mode_keyboard_toggles
                    .in_set(ActiveAfterLoadingSet),
            )
            .add_systems(
                PostUpdate,
                (
                    ball_motion_direction_gizmos.run_if(
                        |debug_mode: Res<DebugMode>| debug_mode.has_ball_motion,
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
                    .after(ActiveAfterLoadingSet),
            );
    }
}

/// Toggles displaying various debugging gizmos.
#[derive(Debug, Default, Resource)]
pub struct DebugMode {
    pub has_ball_motion: bool,
    pub has_crab_stop_positions: bool,
    pub has_crab_ai_ball_targeting: bool,
    pub has_crab_ai_ideal_ball_hit_area: bool,
    pub has_crab_collider_ball_deflection_direction: bool,
}

fn handle_debug_mode_keyboard_toggles(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut debug_mode: ResMut<DebugMode>,
) {
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        let toggle = !debug_mode.has_ball_motion;

        debug_mode.has_ball_motion = toggle;
        debug_mode.has_crab_stop_positions = toggle;
        debug_mode.has_crab_ai_ball_targeting = toggle;
        debug_mode.has_crab_ai_ideal_ball_hit_area = toggle;
        debug_mode.has_crab_collider_ball_deflection_direction = toggle;
    } else if keyboard_input.just_pressed(KeyCode::Digit2) {
        debug_mode.has_ball_motion = !debug_mode.has_ball_motion;
    } else if keyboard_input.just_pressed(KeyCode::Digit3) {
        debug_mode.has_crab_stop_positions =
            !debug_mode.has_crab_stop_positions;
    } else if keyboard_input.just_pressed(KeyCode::Digit4) {
        debug_mode.has_crab_ai_ball_targeting =
            !debug_mode.has_crab_ai_ball_targeting;
    } else if keyboard_input.just_pressed(KeyCode::Digit5) {
        debug_mode.has_crab_ai_ideal_ball_hit_area =
            !debug_mode.has_crab_ai_ideal_ball_hit_area;
    } else if keyboard_input.just_pressed(KeyCode::Digit6) {
        debug_mode.has_crab_collider_ball_deflection_direction =
            !debug_mode.has_crab_collider_ball_deflection_direction;
    }
}

fn ball_motion_direction_gizmos(
    balls_query: Query<
        (&GlobalTransform, &Direction),
        (With<Ball>, With<Motion>),
    >,
    mut gizmos: Gizmos,
) {
    for (global_transform, direction) in &balls_query {
        gizmos.line(
            global_transform.translation(),
            global_transform.translation() + direction.0 * DEBUGGING_RAY_LENGTH,
            Srgba::RED,
        )
    }
}

fn crab_stop_position_gizmos(
    crabs_query: Query<
        (&GlobalTransform, &Direction, &StoppingDistance),
        (With<Crab>, With<Motion>),
    >,
    mut gizmos: Gizmos,
) {
    for (global_transform, direction, stopping_distance) in &crabs_query {
        let mut stop_position_transform = global_transform.compute_transform();
        let global_direction = stop_position_transform.rotation * direction.0;

        stop_position_transform.translation +=
            global_direction * stopping_distance.0;
        gizmos.line(
            global_transform.translation(),
            stop_position_transform.translation,
            Srgba::BLUE,
        );
        gizmos.cuboid(stop_position_transform, Srgba::GREEN);
    }
}

fn crab_ai_ball_targeting_gizmos(
    crabs_query: Query<
        (&GlobalTransform, &Target),
        (With<AI>, With<Crab>, With<Motion>),
    >,
    balls_query: Query<
        &GlobalTransform,
        (With<Ball>, With<Motion>, With<Collider>),
    >,
    mut gizmos: Gizmos,
) {
    for (crab_transform, target) in &crabs_query {
        if let Ok(ball_transform) = balls_query.get(target.0) {
            gizmos.line(
                crab_transform.translation(),
                ball_transform.translation(),
                Srgba::hex("FF00FF").unwrap(),
            );
        }
    }
}

fn crab_ai_ideal_ball_hit_area_gizmos(
    crabs_query: Query<
        (&GlobalTransform, &CrabCollider),
        (With<AI>, With<Crab>, With<Motion>),
    >,
    mut gizmos: Gizmos,
) {
    for (global_transform, crab_collider) in &crabs_query {
        let mut hit_area_transform = global_transform.compute_transform();

        hit_area_transform.scale.x =
            IDEAL_HIT_AREA_PERCENTAGE * crab_collider.width;
        gizmos.cuboid(hit_area_transform, Srgba::hex("FFFF00").unwrap());
    }
}

fn crab_collider_ball_deflection_direction_gizmos(
    goals: Goals,
    crabs_query: Query<
        (&Parent, &Transform, &GlobalTransform, &CrabCollider),
        (With<Crab>, With<Collider>),
    >,
    balls_query: Query<
        (&GlobalTransform, &Direction),
        (With<Ball>, With<Motion>, With<Collider>),
    >,
    mut gizmos: Gizmos,
) {
    for (parent, crab_transform, crab_global_transform, crab_collider) in
        &crabs_query
    {
        let Ok(goal) = goals.get(parent.get()) else {
            continue;
        };

        for (global_transform, direction) in &balls_query {
            if !goal.is_facing(direction) {
                continue;
            }

            // Check that the ball is close enough to the crab.
            let ball_local_x = goal.map_to_local_x(global_transform);
            let crab_translation = crab_global_transform.translation();
            let ball_to_crab_distance =
                global_transform.translation().distance(crab_translation);

            if ball_to_crab_distance > 0.25 {
                continue;
            }

            // Get ball deflection direction.
            let ball_delta_x = crab_transform.translation.x - ball_local_x;
            let new_ball_direction = crab_collider.deflect(&goal, ball_delta_x);

            gizmos.line(
                crab_translation,
                crab_translation + DEBUGGING_RAY_LENGTH * new_ball_direction,
                Srgba::WHITE,
            );
        }
    }
}

// TODO: Add debug visualizations for bounding shapes?
