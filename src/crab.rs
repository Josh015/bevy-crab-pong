use bevy::prelude::*;

use crate::{
    ball::Ball,
    beach::BEACH_CENTER_POINT,
    collider::{Collider, ColliderSet},
    fade::Fade,
    goal::{
        GoalEliminatedEvent, GOAL_CRAB_MAX_POSITION_RANGE,
        GOAL_CRAB_MAX_POSITION_X,
    },
    movement::{Force, Movement, MovementSet, Speed, StoppingDistance},
    side::Side,
    state::AppState,
};

pub const CRAB_WIDTH: f32 = 0.2;
pub const CRAB_DEPTH: f32 = 0.1;
pub const CRAB_HALF_WIDTH: f32 = 0.5 * CRAB_WIDTH;
pub const CRAB_HALF_DEPTH: f32 = 0.5 * CRAB_DEPTH;
pub const CRAB_SCALE: Vec3 = Vec3::new(CRAB_WIDTH, CRAB_DEPTH, CRAB_DEPTH);
pub const CRAB_CENTER_HIT_AREA_PERCENTAGE: f32 = 0.5;

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

// A crab's HP which controls when they are eliminated and the game is over.
#[derive(Clone, Component, Debug)]
pub struct HitPoints(pub u8);

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
                    .run_if(in_state(AppState::Playing)),
                restrict_crabs_to_open_space_in_their_goals
                    .after(MovementSet)
                    .run_if(not(in_state(AppState::Loading)))
                    .run_if(not(in_state(AppState::Paused))),
            ),
        )
        .add_systems(
            PostUpdate,
            deduct_crab_hp_and_potentially_eliminate_goal
                .after(ColliderSet)
                .run_if(in_state(AppState::Playing)),
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
        let mut target_goal_position = BEACH_CENTER_POINT.x;

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

fn restrict_crabs_to_open_space_in_their_goals(
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut Transform, &mut Speed, &mut StoppingDistance),
        (With<Crab>, With<Movement>),
    >,
) {
    for (entity, mut transform, mut speed, mut stopping_distance) in &mut query
    {
        // Limit crab to bounds of the goal.
        if !GOAL_CRAB_MAX_POSITION_RANGE.contains(&transform.translation.x) {
            transform.translation.x = transform
                .translation
                .x
                .clamp(-GOAL_CRAB_MAX_POSITION_X, GOAL_CRAB_MAX_POSITION_X);
            speed.0 = 0.0;
            commands.entity(entity).remove::<Force>();
        }

        // Limit stopping distance to the bounds of the goal.
        let stopped_position = transform.translation.x + stopping_distance.0;

        if !GOAL_CRAB_MAX_POSITION_RANGE.contains(&stopped_position) {
            stopping_distance.0 = stopped_position.signum()
                * GOAL_CRAB_MAX_POSITION_X
                - transform.translation.x;
        }
    }
}

fn deduct_crab_hp_and_potentially_eliminate_goal(
    mut commands: Commands,
    mut goal_eliminated_events: EventWriter<GoalEliminatedEvent>,
    balls_query: Query<
        (Entity, &GlobalTransform),
        (With<Ball>, With<Movement>, With<Collider>),
    >,
    mut crabs_query: Query<(&Parent, &mut HitPoints, &Side), With<Crab>>,
) {
    for (ball_entity, global_transform) in &balls_query {
        for (parent, mut hit_points, side) in &mut crabs_query {
            // A ball will score against the goal it's closest to once it's
            // fully past the goal's crab.
            let ball_distance = side.distance_to_ball(global_transform);

            if ball_distance > -CRAB_HALF_DEPTH {
                continue;
            }

            // Decrement the crab's HP and potentially eliminate it.
            hit_points.0 = hit_points.0.saturating_sub(1);
            info!("Ball({:?}): Scored Goal({:?})", ball_entity, side);

            if hit_points.0 == 0 {
                goal_eliminated_events.send(GoalEliminatedEvent(parent.get()));
                info!("Ball({:?}): Eliminated Goal({:?})", ball_entity, side);
            }

            // Despawn and replace the scoring ball.
            commands.entity(ball_entity).insert(Fade::out_default());
            break;
        }
    }
}
