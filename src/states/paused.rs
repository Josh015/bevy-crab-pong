use bevy::prelude::*;

use crate::{
    events::MessageUiEvent,
    serialization::{GameAssets, GameConfig},
};

use super::GameState;

fn spawn_pause_ui(
    game_assets: Res<GameAssets>,
    game_configs: Res<Assets<GameConfig>>,
    mut ui_message_events: EventWriter<MessageUiEvent>,
) {
    let game_config = game_configs.get(&game_assets.game_config).unwrap();

    ui_message_events.send(MessageUiEvent {
        message: game_config.pause_message.clone(),
        game_state: GameState::Paused,
    });
}

pub struct PausedPlugin;

impl Plugin for PausedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Paused), spawn_pause_ui);
    }
}
