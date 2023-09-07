#![allow(clippy::type_complexity)]

use crate::prelude::*;
use std::ops::RangeInclusive;

pub const GOAL_WIDTH: f32 = 1.0;
pub const GOAL_HALF_WIDTH: f32 = 0.5 * GOAL_WIDTH;
pub const GOAL_PADDLE_START_POSITION: Vec3 = Vec3::new(0.0, 0.05, 0.0);
pub const GOAL_PADDLE_MAX_POSITION_X: f32 =
    GOAL_HALF_WIDTH - BARRIER_RADIUS - PADDLE_HALF_WIDTH;
pub const GOAL_PADDLE_MAX_POSITION_RANGE: RangeInclusive<f32> =
    -GOAL_PADDLE_MAX_POSITION_X..=GOAL_PADDLE_MAX_POSITION_X;

/// An event fired when a [`Goal`] has been eliminated from play after its HP
/// has reached zero.
#[derive(Event)]
pub struct GoalEliminatedEvent(pub Side);

/// Marks a [`Goal`] entity so that [`Paddle`] and [`Wall`] entities can use it
/// as a parent, and so [`Ball`] entities can score against it.
#[derive(Component)]
pub struct Goal;

/// Checks if a [`Ball`] has scored against a [`Goal`] and then decrements the
/// corresponding score.
fn goal_scored_check(
    mut commands: Commands,
    mut run_state: ResMut<RunState>,
    mut fade_out_entity_events: EventWriter<FadeOutEntityEvent>,
    mut goal_eliminated_writer: EventWriter<GoalEliminatedEvent>,
    balls_query: Query<
        (Entity, &GlobalTransform),
        (With<Ball>, With<Collider>),
    >,
    goals_query: Query<&Side, With<Goal>>,
) {
    for (ball_entity, global_transform) in &balls_query {
        for side in &goals_query {
            // A ball will score against the goal it's closest to once it's
            // fully past the goal's paddle.
            let ball_distance = side.distance_to_ball(global_transform);

            if ball_distance > -PADDLE_HALF_DEPTH {
                continue;
            }

            // Decrement the goal's HP and potentially eliminate it.
            let hit_points = run_state.goals_hit_points.get_mut(side).unwrap();

            *hit_points = hit_points.saturating_sub(1);
            info!("Ball({:?}): Scored Goal({:?})", ball_entity, side);

            if *hit_points == 0 {
                goal_eliminated_writer.send(GoalEliminatedEvent(*side));
                info!("Ball({:?}): Eliminated Goal({:?})", ball_entity, side);
            }

            // Remove Collider and start fading out the ball to prevent
            // repeated scoring.
            commands.entity(ball_entity).remove::<Collider>();
            fade_out_entity_events.send(FadeOutEntityEvent(ball_entity));
            break;
        }
    }
}

/// Disables a given [`Goal`] to remove it from play.
fn goal_eliminated_event(
    mut commands: Commands,
    mut event_reader: EventReader<GoalEliminatedEvent>,
    mut fade_out_entity_events: EventWriter<FadeOutEntityEvent>,
    mut spawn_wall_events: EventWriter<SpawnWallEvent>,
    paddles_query: Query<
        (Entity, &Side),
        (With<Paddle>, With<Collider>, Without<Fade>),
    >,
) {
    for GoalEliminatedEvent(eliminated_side) in event_reader.iter() {
        // Fade out the paddle for the eliminated goal.
        for (paddle_entity, side) in &paddles_query {
            if *side != *eliminated_side {
                continue;
            }

            // Stop the paddle from moving and colliding.
            commands
                .entity(paddle_entity)
                .remove::<(Collider, VelocityBundle)>();
            fade_out_entity_events.send(FadeOutEntityEvent(paddle_entity));
            break;
        }

        // Fade in the wall for the eliminated goal.
        spawn_wall_events.send(SpawnWallEvent {
            side: *eliminated_side,
            is_instant: false,
        });
    }
}

pub struct GoalPlugin;

impl Plugin for GoalPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GoalEliminatedEvent>().add_systems(
            Update,
            (goal_scored_check, goal_eliminated_event)
                .chain()
                .in_set(GameSystemSet::GameplayLogic),
        );
    }
}
