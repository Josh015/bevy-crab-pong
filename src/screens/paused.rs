use crate::{config::GameConfig, events::MessageUiEvent, screens::GameScreen};
use bevy::prelude::*;

fn spawn_pause_ui(
    game_config: Res<GameConfig>,
    mut ui_message_events: EventWriter<MessageUiEvent>,
) {
    ui_message_events.send(MessageUiEvent {
        message: game_config.pause_message.clone(),
        game_screen: GameScreen::Paused,
    });
}

pub struct PausedPlugin;

impl Plugin for PausedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameScreen::Paused), spawn_pause_ui);
    }
}
