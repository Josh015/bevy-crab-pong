use bevy::prelude::{App, Entity, Event, Plugin};

use crate::{components::goals::Side, screens::GameScreen};

/// Objects that can be spawned via `SpawnEvent`.
#[derive(Debug, Eq, PartialEq)]
pub enum Object {
    Ball,
    Wall,
    Paddle,
}

/// An event fired when spawning a message UI.
#[derive(Event)]
pub struct MessageUiEvent {
    pub message: String,
    pub game_screen: GameScreen,
}

/// Removes a goal's current paddle or wall child entity.
#[derive(Event)]
pub struct RemoveGoalOccupantEvent(pub Entity);

/// An event fired when a [`Goal`] has been eliminated from play after its HP
/// has reached zero.
#[derive(Event)]
pub struct GoalEliminatedEvent(pub Side);

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MessageUiEvent>()
            .add_event::<GoalEliminatedEvent>()
            .add_event::<RemoveGoalOccupantEvent>();
    }
}
