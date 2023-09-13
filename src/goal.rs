use bevy::prelude::*;
use spew::prelude::SpawnEvent;
use std::ops::RangeInclusive;

use crate::{
    barrier::BARRIER_RADIUS, object::Object, paddle::PADDLE_HALF_WIDTH,
};

pub const GOAL_WIDTH: f32 = 1.0;
pub const GOAL_HALF_WIDTH: f32 = 0.5 * GOAL_WIDTH;
pub const GOAL_PADDLE_START_POSITION: Vec3 = Vec3::new(0.0, 0.05, 0.0);
pub const GOAL_PADDLE_MAX_POSITION_X: f32 =
    GOAL_HALF_WIDTH - BARRIER_RADIUS - PADDLE_HALF_WIDTH;
pub const GOAL_PADDLE_MAX_POSITION_RANGE: RangeInclusive<f32> =
    -GOAL_PADDLE_MAX_POSITION_X..=GOAL_PADDLE_MAX_POSITION_X;

/// Marks a goal entity so that paddles and walls can use it as a parent, and
/// so balls can score against it.
#[derive(Component, Debug)]
pub struct Goal;

/// Signals a goal being eliminated from the game.
#[derive(Clone, Component, Debug, Event)]
pub struct GoalEliminatedEvent(pub Entity);

pub struct GoalPlugin;

impl Plugin for GoalPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GoalEliminatedEvent>()
            .add_systems(PostUpdate, block_eliminated_goals_with_walls);
    }
}

fn block_eliminated_goals_with_walls(
    mut goal_eliminated_events: EventReader<GoalEliminatedEvent>,
    mut spawn_in_goal_events: EventWriter<SpawnEvent<Object, Entity>>,
) {
    for GoalEliminatedEvent(entity) in goal_eliminated_events.iter() {
        spawn_in_goal_events.send(SpawnEvent::with_data(Object::Wall, *entity));
    }
}
