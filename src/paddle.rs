use bevy::prelude::*;

use crate::{
    arena::ARENA_CENTER_POINT,
    ball::{Ball, BallSet},
    goal::{
        GoalEliminatedEvent, GOAL_PADDLE_MAX_POSITION_RANGE,
        GOAL_PADDLE_MAX_POSITION_X,
    },
    movement::{Force, MovementSet, Speed, StoppingDistance},
    side::Side,
    spawning::{Despawning, DespawningBundle, Spawning},
    state::AppState,
};

pub const PADDLE_WIDTH: f32 = 0.2;
pub const PADDLE_DEPTH: f32 = 0.1;
pub const PADDLE_HALF_WIDTH: f32 = 0.5 * PADDLE_WIDTH;
pub const PADDLE_HALF_DEPTH: f32 = 0.5 * PADDLE_DEPTH;
pub const PADDLE_SCALE: Vec3 =
    Vec3::new(PADDLE_WIDTH, PADDLE_DEPTH, PADDLE_DEPTH);
pub const PADDLE_CENTER_HIT_AREA_PERCENTAGE: f32 = 0.5;

/// Makes an entity that can move along a single access inside a goal.
#[derive(Component, Debug)]
pub struct Paddle;

/// Marks a [`Paddle`] entity as being controlled by the keyboard.
#[derive(Component, Debug)]
pub struct KeyboardPlayer;

/// Marks a [`Paddle`] entity as being controlled by AI.
#[derive(Component, Debug)]
pub struct AiPlayer;

/// The [`Ball`] entity targeted by an [`AiPlayer`] [`Paddle`] entity.
#[derive(Clone, Component, Debug)]
#[component(storage = "SparseSet")]
pub struct Target(pub Entity);

// A paddle's HP which controls when they are eliminated and the game is over.
#[derive(Clone, Component, Debug)]
pub struct HitPoints(pub u8);

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (
                    handle_keyboard_input_for_player_controlled_paddles,
                    make_ai_paddles_target_the_balls_closest_to_their_goals,
                    move_ai_paddles_toward_their_targeted_balls,
                )
                    .chain()
                    .before(MovementSet)
                    .run_if(in_state(AppState::Playing)),
                restrict_paddles_to_open_space_in_their_goals
                    .after(MovementSet)
                    .run_if(not(in_state(AppState::Loading)))
                    .run_if(not(in_state(AppState::Paused))),
            ),
        )
        .add_systems(
            PostUpdate,
            deduct_paddle_hp_and_potentially_eliminate_goal
                .after(BallSet)
                .run_if(in_state(AppState::Playing)),
        );
    }
}
fn handle_keyboard_input_for_player_controlled_paddles(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    paddles_query: Query<
        Entity,
        (
            With<Paddle>,
            With<KeyboardPlayer>,
            Without<Spawning>,
            Without<Despawning>,
        ),
    >,
) {
    // Makes a Paddle entity move left/right in response to the
    // keyboard's corresponding arrows keys.
    for entity in &paddles_query {
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

fn make_ai_paddles_target_the_balls_closest_to_their_goals(
    mut commands: Commands,
    paddles_query: Query<
        (Entity, &Side),
        (
            With<Paddle>,
            With<AiPlayer>,
            Without<Spawning>,
            Without<Despawning>,
        ),
    >,
    balls_query: Query<
        (Entity, &GlobalTransform),
        (With<Ball>, Without<Spawning>, Without<Despawning>),
    >,
) {
    for (paddle_entity, side) in &paddles_query {
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
            commands.entity(paddle_entity).insert(Target(target));
        } else {
            commands.entity(paddle_entity).remove::<Target>();
        }
    }
}

fn move_ai_paddles_toward_their_targeted_balls(
    mut commands: Commands,
    paddles_query: Query<
        (
            Entity,
            &Side,
            &Transform,
            &StoppingDistance,
            Option<&Target>,
        ),
        (
            With<Paddle>,
            With<AiPlayer>,
            Without<Spawning>,
            Without<Despawning>,
        ),
    >,
    balls_query: Query<
        &GlobalTransform,
        (With<Ball>, Without<Spawning>, Without<Despawning>),
    >,
) {
    for (entity, side, transform, stopping_distance, target) in &paddles_query {
        // Use the ball's goal position or default to the center of the goal.
        let mut target_goal_position = ARENA_CENTER_POINT.x;

        if let Some(target) = target {
            if let Ok(ball_transform) = balls_query.get(target.0) {
                target_goal_position = side.get_ball_position(ball_transform)
            }
        }

        // Move the paddle so that its center is over the target goal position.
        let paddle_stop_position =
            transform.translation.x + stopping_distance.0;
        let distance_from_paddle_center =
            (paddle_stop_position - target_goal_position).abs();

        if distance_from_paddle_center
            < PADDLE_CENTER_HIT_AREA_PERCENTAGE * PADDLE_HALF_WIDTH
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

fn restrict_paddles_to_open_space_in_their_goals(
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut Transform, &mut Speed, &mut StoppingDistance),
        (With<Paddle>, Without<Spawning>),
    >,
) {
    for (entity, mut transform, mut speed, mut stopping_distance) in &mut query
    {
        // Limit paddle to bounds of the goal.
        if !GOAL_PADDLE_MAX_POSITION_RANGE.contains(&transform.translation.x) {
            transform.translation.x = transform
                .translation
                .x
                .clamp(-GOAL_PADDLE_MAX_POSITION_X, GOAL_PADDLE_MAX_POSITION_X);
            speed.0 = 0.0;
            commands.entity(entity).remove::<Force>();
        }

        // Limit stopping distance to the bounds of the goal.
        let stopped_position = transform.translation.x + stopping_distance.0;

        if !GOAL_PADDLE_MAX_POSITION_RANGE.contains(&stopped_position) {
            stopping_distance.0 = stopped_position.signum()
                * GOAL_PADDLE_MAX_POSITION_X
                - transform.translation.x;
        }
    }
}

fn deduct_paddle_hp_and_potentially_eliminate_goal(
    mut commands: Commands,
    mut goal_eliminated_events: EventWriter<GoalEliminatedEvent>,
    balls_query: Query<
        (Entity, &GlobalTransform),
        (With<Ball>, Without<Spawning>, Without<Despawning>),
    >,
    mut paddles_query: Query<(&Parent, &mut HitPoints, &Side), With<Paddle>>,
) {
    for (ball_entity, global_transform) in &balls_query {
        for (parent, mut hit_points, side) in &mut paddles_query {
            // A ball will score against the goal it's closest to once it's
            // fully past the goal's paddle.
            let ball_distance = side.distance_to_ball(global_transform);

            if ball_distance > -PADDLE_HALF_DEPTH {
                continue;
            }

            // Decrement the paddle's HP and potentially eliminate it.
            hit_points.0 = hit_points.0.saturating_sub(1);
            info!("Ball({:?}): Scored Goal({:?})", ball_entity, side);

            if hit_points.0 == 0 {
                goal_eliminated_events.send(GoalEliminatedEvent(parent.get()));
                info!("Ball({:?}): Eliminated Goal({:?})", ball_entity, side);
            }

            // Despawn and replace the scoring ball.
            commands
                .entity(ball_entity)
                .insert(DespawningBundle::default());
            break;
        }
    }
}
