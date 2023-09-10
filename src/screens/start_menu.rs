use bevy::prelude::*;
use spew::prelude::SpawnEvent;

use crate::{
    components::goals::*,
    constants::*,
    events::{MessageUiEvent, Object},
    global_data::{GameOver, GlobalData},
    screens::GameScreen,
    serialization::Config,
};

fn spawn_start_menu_ui(
    config: Res<Config>,
    global_data: Res<GlobalData>,
    mut ui_message_events: EventWriter<MessageUiEvent>,
) {
    let mut message = String::from(match global_data.game_over {
        Some(GameOver::Won) => &config.game_over_win_message,
        Some(GameOver::Lost) => &config.game_over_lose_message,
        _ => "",
    });

    message.push_str(&config.new_game_message);

    ui_message_events.send(MessageUiEvent {
        message,
        game_screen: GameScreen::StartMenu,
    });
}

fn reset_each_goals_hit_points(
    config: Res<Config>,
    mut global_data: ResMut<GlobalData>,
) {
    const SIDES: [Side; 4] = [Side::Top, Side::Right, Side::Bottom, Side::Left];
    let goals = &config.modes[global_data.mode_index].goals;

    for (i, side) in SIDES.iter().enumerate() {
        global_data
            .goals_hit_points
            .insert(*side, goals[i].starting_hit_points);
    }
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
                (
                    reset_each_goals_hit_points,
                    give_each_goal_a_new_paddle,
                    spawn_starting_balls,
                )
                    .chain(),
            );
    }
}
