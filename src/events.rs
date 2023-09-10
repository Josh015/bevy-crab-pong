use bevy::prelude::{App, Event, Plugin};

use crate::screens::GameScreen;

/// An event fired when spawning a message UI.
#[derive(Event, Debug)]
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
