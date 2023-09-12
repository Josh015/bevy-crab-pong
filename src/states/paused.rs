use bevy::prelude::*;

use crate::{
    hud::MessageUiEvent,
    resources::{GameAssets, GameConfig},
    state::AppState,
};

fn spawn_pause_ui(
    game_assets: Res<GameAssets>,
    game_configs: Res<Assets<GameConfig>>,
    mut ui_message_events: EventWriter<MessageUiEvent>,
) {
    let game_config = game_configs.get(&game_assets.game_config).unwrap();

    ui_message_events.send(MessageUiEvent {
        message: game_config.pause_message.clone(),
        game_state: AppState::Paused,
    });
}

pub struct PausedPlugin;

impl Plugin for PausedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Paused), spawn_pause_ui);
    }
}
