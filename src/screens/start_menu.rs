use bevy::prelude::*;
use spew::prelude::SpawnEvent;

use crate::{
    components::{
        goals::{Goal, Side},
        spawning::Object,
    },
    constants::*,
    events::MessageUiEvent,
    global_data::GlobalData,
    serialization::Config,
};

use super::GameScreen;

fn spawn_start_menu_ui(
    config: Res<Config>,
    global_data: Res<GlobalData>,
    mut ui_message_events: EventWriter<MessageUiEvent>,
) {
    let mut message = String::from(match global_data.winning_team {
        Some(winning_team) => &config.team_win_messages[winning_team],
        _ => "",
    });

    message.push_str(&config.new_game_message);

    ui_message_events.send(MessageUiEvent {
        message,
        game_screen: GameScreen::StartMenu,
    });
}

fn give_each_goal_a_new_paddle(
    goals_query: Query<&Side, With<Goal>>,
    mut spawn_in_goal_events: EventWriter<SpawnEvent<Object, Side>>,
) {
    for side in &goals_query {
        spawn_in_goal_events.send(SpawnEvent::with_data(Object::Paddle, *side));
    }
}

fn spawn_starting_balls(
    config: Res<Config>,
    global_data: Res<GlobalData>,
    mut spawn_events: EventWriter<SpawnEvent<Object>>,
) {
    for i in 0..config.modes[global_data.mode_index].max_ball_count {
        spawn_events.send(
            SpawnEvent::new(Object::Ball)
                .delay_seconds(i as f32 * BALL_SPAWN_DELAY_IN_SECONDS),
        );
    }
}

pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameScreen::StartMenu), spawn_start_menu_ui)
            .add_systems(
                OnExit(GameScreen::StartMenu),
                (give_each_goal_a_new_paddle, spawn_starting_balls).chain(),
            );
    }
}
