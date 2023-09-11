use bevy::prelude::{App, Event, Plugin};

use crate::states::GameState;

/// An event fired when spawning a message UI.
#[derive(Event, Debug)]
pub struct MessageUiEvent {
    pub message: String,
    pub game_state: GameState,
}

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MessageUiEvent>();
    }
}
