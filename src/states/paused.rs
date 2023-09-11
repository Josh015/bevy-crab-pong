use bevy::prelude::*;

use crate::{events::MessageUiEvent, serialization::Config};

use super::GameState;

fn spawn_pause_ui(
    config: Res<Config>,
    mut ui_message_events: EventWriter<MessageUiEvent>,
) {
    ui_message_events.send(MessageUiEvent {
        message: config.pause_message.clone(),
        game_state: GameState::Paused,
    });
}

pub struct PausedPlugin;

impl Plugin for PausedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Paused), spawn_pause_ui);
    }
}
