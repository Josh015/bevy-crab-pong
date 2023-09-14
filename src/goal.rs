use bevy::prelude::*;
use spew::prelude::*;
use std::ops::RangeInclusive;

use crate::{
    ball::Ball,
    barrier::BARRIER_RADIUS,
    collider::{Collider, ColliderSet},
    crab::{Crab, CRAB_HALF_DEPTH, CRAB_HALF_WIDTH},
    fade::Fade,
    movement::Movement,
    object::Object,
    side::Side,
    wall::Wall,
};

pub const GOAL_WIDTH: f32 = 1.0;
pub const GOAL_HALF_WIDTH: f32 = 0.5 * GOAL_WIDTH;
pub const GOAL_CRAB_START_POSITION: Vec3 = Vec3::new(0.0, 0.05, 0.0);
pub const GOAL_CRAB_MAX_POSITION_X: f32 =
    GOAL_HALF_WIDTH - BARRIER_RADIUS - CRAB_HALF_WIDTH;
pub const GOAL_CRAB_MAX_POSITION_RANGE: RangeInclusive<f32> =
    -GOAL_CRAB_MAX_POSITION_X..=GOAL_CRAB_MAX_POSITION_X;

/// Marks a goal entity so that crabs and walls can use it as a parent, and
/// so balls can score against it.
#[derive(Component, Debug)]
pub struct Goal;

/// Signals a goal being scored in by a ball.
#[derive(Clone, Component, Debug, Event)]
pub struct GoalScoredEvent(pub Side);

/// Signals a goal being eliminated from the game.
#[derive(Clone, Component, Debug, Event)]
pub struct GoalEliminatedEvent(pub Side);

pub struct GoalPlugin;

impl Plugin for GoalPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GoalScoredEvent>()
            .add_event::<GoalEliminatedEvent>()
            .add_systems(
                Update,
                allow_only_one_crab_or_wall_in_a_goal.after(SpewSystemSet),
            )
            .add_systems(
                PostUpdate,
                (
                    check_if_any_balls_have_scored_in_any_goals,
                    block_eliminated_goals_with_walls,
                )
                    .after(ColliderSet),
            );
    }
}

fn allow_only_one_crab_or_wall_in_a_goal(
    mut commands: Commands,
    new_query: Query<(Entity, &Parent), Or<(Added<Crab>, Added<Wall>)>>,
    old_query: Query<(Entity, &Parent), Or<(With<Crab>, With<Wall>)>>,
) {
    for (new_entity, new_parent) in &new_query {
        for (old_entity, old_parent) in &old_query {
            if old_parent == new_parent && old_entity != new_entity {
                commands
                    .entity(old_entity)
                    .remove::<Movement>()
                    .insert(Fade::out_default());
                break;
            }
        }
    }
}

fn check_if_any_balls_have_scored_in_any_goals(
    mut commands: Commands,
    mut goal_scored_events: EventWriter<GoalScoredEvent>,
    balls_query: Query<
        (Entity, &GlobalTransform),
        (With<Ball>, With<Movement>, With<Collider>),
    >,
    crabs_query: Query<&Side, (With<Crab>, With<Movement>, With<Collider>)>,
) {
    // If a ball passes a goal's alive crab then despawn it and raise an event.
    for (ball_entity, global_transform) in &balls_query {
        for side in &crabs_query {
            let ball_distance = side.distance_to_ball(global_transform);

            if ball_distance <= -CRAB_HALF_DEPTH {
                commands.entity(ball_entity).insert(Fade::out_default());
                goal_scored_events.send(GoalScoredEvent(*side));
                info!("Ball({:?}): Scored Goal({:?})", ball_entity, side);
            }
        }
    }
}

fn block_eliminated_goals_with_walls(
    mut goal_eliminated_events: EventReader<GoalEliminatedEvent>,
    mut spawn_on_side_events: EventWriter<SpawnEvent<Object, Side>>,
) {
    for GoalEliminatedEvent(side) in goal_eliminated_events.iter() {
        spawn_on_side_events.send(SpawnEvent::with_data(Object::Wall, *side));
        info!("Goal({:?}): Eliminated", side);
    }
}
