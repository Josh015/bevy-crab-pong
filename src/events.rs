use crate::{components::goals::Side, screens::GameScreen};
use bevy::prelude::{App, Entity, Event, Plugin};

/// An event fired when spawning a message UI.
#[derive(Event)]
pub struct MessageUiEvent {
    pub message: String,
    pub game_screen: GameScreen,
}

/// An event fired when a [`Goal`] has been eliminated from play after its HP
/// has reached zero.
#[derive(Event)]
pub struct GoalEliminatedEvent(pub Side);

/// An event fired when a [`Wall`] needs to be spawned.
#[derive(Event)]
pub struct SpawnWallEvent {
    pub side: Side,
    pub is_instant: bool,
}

#[derive(Event)]
pub struct FadeOutEntityEvent(pub Entity);

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MessageUiEvent>()
            .add_event::<GoalEliminatedEvent>()
            .add_event::<SpawnWallEvent>()
            .add_event::<FadeOutEntityEvent>();
    }
}
