use bevy::prelude::*;
use std::ops::RangeInclusive;

use crate::{
    ball::Ball,
    barrier::BARRIER_RADIUS,
    collider::{Collider, ColliderSet},
    debug_mode::DebugModeSet,
    goal::GOAL_HALF_WIDTH,
    movement::{
        Force, Heading, Movement, MovementSet, Speed, StoppingDistance,
    },
    side::Side,
    state::GameState,
};

pub const CRAB_WIDTH: f32 = 0.2;
pub const CRAB_DEPTH: f32 = 0.1;
pub const CRAB_HALF_WIDTH: f32 = 0.5 * CRAB_WIDTH;
pub const CRAB_HALF_DEPTH: f32 = 0.5 * CRAB_DEPTH;
pub const CRAB_SCALE: Vec3 = Vec3::new(CRAB_WIDTH, CRAB_DEPTH, CRAB_DEPTH);
pub const CRAB_CENTER_HIT_AREA_PERCENTAGE: f32 = 0.5;
pub const CRAB_START_POSITION: Vec3 = Vec3::new(0.0, 0.05, 0.0);
pub const CRAB_POSITION_X_MAX: f32 =
    GOAL_HALF_WIDTH - BARRIER_RADIUS - CRAB_HALF_WIDTH;
pub const CRAB_POSITION_X_MAX_RANGE: RangeInclusive<f32> =
    -CRAB_POSITION_X_MAX..=CRAB_POSITION_X_MAX;

/// Makes a crab entity that can deflect balls and move sideways inside a goal.
#[derive(Component, Debug)]
pub struct Crab;

/// Marks a [`Crab`] entity as being controlled by the keyboard.
#[derive(Component, Debug)]
pub struct KeyboardPlayer;

/// Marks a [`Crab`] entity as being controlled by AI.
#[derive(Component, Debug)]
pub struct AiPlayer;

/// The [`Ball`] entity targeted by an [`AiPlayer`] [`Crab`] entity.
#[derive(Clone, Component, Debug)]
#[component(storage = "SparseSet")]
pub struct Target(pub Entity);

pub struct CrabPlugin;

impl Plugin for CrabPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (
                    handle_keyboard_input_for_player_controlled_crabs,
                    make_ai_crabs_target_the_balls_closest_to_their_goals,
                    move_ai_crabs_toward_their_targeted_balls,
                )
                    .chain()
                    .before(MovementSet)
                    .run_if(in_state(GameState::Playing)),
                restrict_crab_movement_range.after(MovementSet),
            ),
        )
        .add_systems(
            PostUpdate,
            (
                crab_and_ball_collisions.in_set(ColliderSet),
                (
                    display_crab_predicted_stop_position_gizmos,
                    display_crab_to_ball_targeting_gizmos,
                    display_ai_crab_ideal_hit_area_gizmos,
                )
                    .in_set(DebugModeSet),
            ),
        );
    }
}

fn handle_keyboard_input_for_player_controlled_crabs(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    crabs_query: Query<
        Entity,
        (With<Crab>, With<KeyboardPlayer>, With<Movement>),
    >,
) {
    // Makes a Crab entity move left/right in response to the
    // keyboard's corresponding arrows keys.
    for entity in &crabs_query {
        if keyboard_input.pressed(KeyCode::Left)
            || keyboard_input.pressed(KeyCode::A)
        {
            commands.entity(entity).insert(Force::Negative);
        } else if keyboard_input.pressed(KeyCode::Right)
            || keyboard_input.pressed(KeyCode::D)
        {
            commands.entity(entity).insert(Force::Positive);
        } else {
            commands.entity(entity).remove::<Force>();
        };
    }

    // TODO: Need to make inputs account for side!
}

fn make_ai_crabs_target_the_balls_closest_to_their_goals(
    mut commands: Commands,
    crabs_query: Query<
        (Entity, &Side),
        (With<Crab>, With<AiPlayer>, With<Movement>),
    >,
    balls_query: Query<
        (Entity, &GlobalTransform),
        (With<Ball>, With<Movement>, With<Collider>),
    >,
) {
    for (crab_entity, side) in &crabs_query {
        let mut closest_ball_distance = std::f32::MAX;
        let mut target = None;

        for (ball_entity, ball_transform) in &balls_query {
            let ball_distance_to_goal = side.distance_to_ball(ball_transform);

            if ball_distance_to_goal < closest_ball_distance {
                closest_ball_distance = ball_distance_to_goal;
                target = Some(ball_entity);
            }
        }

        if let Some(target) = target {
            commands.entity(crab_entity).insert(Target(target));
        } else {
            commands.entity(crab_entity).remove::<Target>();
        }
    }
}

fn move_ai_crabs_toward_their_targeted_balls(
    mut commands: Commands,
    crabs_query: Query<
        (
            Entity,
            &Side,
            &Transform,
            &StoppingDistance,
            Option<&Target>,
        ),
        (With<Crab>, With<AiPlayer>, With<Movement>),
    >,
    balls_query: Query<
        &GlobalTransform,
        (With<Ball>, With<Movement>, With<Collider>),
    >,
) {
    for (entity, side, transform, stopping_distance, target) in &crabs_query {
        // Use the ball's goal position or default to the center of the goal.
        let mut target_goal_position = CRAB_START_POSITION.x;

        if let Some(target) = target {
            if let Ok(ball_transform) = balls_query.get(target.0) {
                target_goal_position = side.get_ball_position(ball_transform)
            }
        }

        // Move the crab so that its center is over the target goal position.
        let crab_stop_position = transform.translation.x + stopping_distance.0;
        let distance_from_crab_center =
            (crab_stop_position - target_goal_position).abs();

        if distance_from_crab_center
            < CRAB_CENTER_HIT_AREA_PERCENTAGE * CRAB_HALF_WIDTH
        {
            commands.entity(entity).remove::<Force>();
        } else {
            commands.entity(entity).insert(
                if target_goal_position < transform.translation.x {
                    Force::Negative // Left
                } else {
                    Force::Positive // Right
                },
            );
        }
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
        if !CRAB_POSITION_X_MAX_RANGE.contains(&transform.translation.x) {
            transform.translation.x = transform
                .translation
                .x
                .clamp(-CRAB_POSITION_X_MAX, CRAB_POSITION_X_MAX);
            speed.0 = 0.0;
            commands.entity(entity).remove::<Force>();
        }

        // Limit stopping distance to the bounds of the goal.
        let stopped_position = transform.translation.x + stopping_distance.0;

        if !CRAB_POSITION_X_MAX_RANGE.contains(&stopped_position) {
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
    for (entity, ball_transform, ball_heading) in &balls_query {
        for (side, transform) in &crabs_query {
            let goal_axis = side.axis();
            let ball_distance_to_goal = side.distance_to_ball(ball_transform);
            let ball_goal_position = side.get_ball_position(ball_transform);
            let ball_to_crab = transform.translation.x - ball_goal_position;
            let ball_distance_to_crab = ball_to_crab.abs();

            // Check that the ball is touching the crab and facing the goal.
            if ball_distance_to_goal > CRAB_HALF_DEPTH
                || ball_distance_to_crab >= CRAB_HALF_WIDTH
                || ball_heading.0.dot(goal_axis) <= 0.0
            {
                continue;
            }

            // Reverse the ball's direction and rotate it outward based on how
            // far its position is from the crab's center.
            let rotation_away_from_center = Quat::from_rotation_y(
                std::f32::consts::FRAC_PI_4 * (ball_to_crab / CRAB_HALF_WIDTH),
            );
            commands
                .entity(entity)
                .insert(Heading(rotation_away_from_center * -ball_heading.0));

            info!("Ball({:?}): Collided Crab({:?})", entity, side);
            break;
        }
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
