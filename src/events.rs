use bevy::prelude::{App, Event, Plugin};

use crate::screens::GameScreen;

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

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MessageUiEvent>();
    }
}
