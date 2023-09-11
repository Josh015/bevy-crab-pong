use bevy::prelude::*;
use spew::prelude::SpawnEvent;

use crate::{
    components::{goals::Goal, spawning::Object},
    events::MessageUiEvent,
    global_data::GlobalData,
    serialization::{GameAssets, GameConfig},
};

use super::GameState;

fn spawn_start_menu_ui(
    game_assets: Res<GameAssets>,
    game_configs: Res<Assets<GameConfig>>,
    global_data: Res<GlobalData>,
    mut ui_message_events: EventWriter<MessageUiEvent>,
) {
    let game_config = game_configs.get(&game_assets.game_config).unwrap();
    let mut message = String::from(match global_data.winning_team {
        Some(winning_team) => &game_config.team_win_messages[winning_team],
        _ => "",
    });

    message.push_str(&game_config.new_game_message);

    ui_message_events.send(MessageUiEvent {
        message,
        game_state: GameState::StartMenu,
    });
}

fn give_each_goal_a_new_paddle(
    goals_query: Query<Entity, With<Goal>>,
    mut spawn_in_goal_events: EventWriter<SpawnEvent<Object, Entity>>,
) {
    for entity in &goals_query {
        spawn_in_goal_events
            .send(SpawnEvent::with_data(Object::Paddle, entity));
    }
}

pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::StartMenu), spawn_start_menu_ui)
            .add_systems(
                OnExit(GameState::StartMenu),
                give_each_goal_a_new_paddle.chain(),
            );
    }
}
