use bevy::prelude::*;

use crate::{
    components::{
        ball::Ball,
        collider::Collider,
        crab::{
            Crab, CrabCollider,
            ai::{AI, IDEAL_HIT_AREA_PERCENTAGE, Target},
        },
        movement::{Heading, Movement, StoppingDistance},
    },
    game::{state::LoadedSet, system_params::Goals},
    util::hemisphere_deflection,
};

pub const DEBUGGING_RAY_LENGTH: f32 = 20.0;

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

/// Toggles displaying various debugging gizmos.
#[derive(Debug, Default, Resource)]
pub struct DebugMode {
    pub has_ball_movement: bool,
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
        let toggle = !debug_mode.has_ball_movement;

        debug_mode.has_ball_movement = toggle;
        debug_mode.has_crab_stop_positions = toggle;
        debug_mode.has_crab_ai_ball_targeting = toggle;
        debug_mode.has_crab_ai_ideal_ball_hit_area = toggle;
        debug_mode.has_crab_collider_ball_deflection_direction = toggle;
    } else if keyboard_input.just_pressed(KeyCode::Digit2) {
        debug_mode.has_ball_movement = !debug_mode.has_ball_movement;
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
            Srgba::RED,
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
            Srgba::BLUE,
        );
        gizmos.cuboid(stop_position_transform, Srgba::GREEN);
    }
}

fn crab_ai_ball_targeting_gizmos(
    crabs_query: Query<
        (&GlobalTransform, &Target),
        (With<AI>, With<Crab>, With<Movement>),
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
                Srgba::hex("FF00FF").unwrap(),
            );
        }
    }
}

fn crab_ai_ideal_ball_hit_area_gizmos(
    crabs_query: Query<
        (&GlobalTransform, &CrabCollider),
        (With<AI>, With<Crab>, With<Movement>),
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
        (&GlobalTransform, &Heading),
        (With<Ball>, With<Movement>, With<Collider>),
    >,
    mut gizmos: Gizmos,
) {
    for (parent, crab_transform, crab_global_transform, crab_collider) in
        &crabs_query
    {
        let Ok(goal) = goals.get(parent.get()) else {
            continue;
        };

        for (global_transform, heading) in &balls_query {
            if !goal.is_facing(heading) {
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
            let delta = crab_transform.translation.x - ball_local_x;
            let ball_deflection_direction =
                hemisphere_deflection(delta, crab_collider.width, goal.forward);

            gizmos.line(
                crab_translation,
                crab_translation
                    + DEBUGGING_RAY_LENGTH * ball_deflection_direction,
                Srgba::WHITE,
            );
        }
    }
}

// TODO: Add debug visualizations for bounding shapes?
