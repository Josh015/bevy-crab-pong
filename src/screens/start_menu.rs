use bevy::prelude::*;
use spew::prelude::SpawnEvent;

use crate::{
    components::{balls::*, goals::*},
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

fn disable_ball_collisions(
    mut commands: Commands,
    balls_query: Query<Entity, (With<Ball>, With<Collider>)>,
) {
    // Ensure balls pass through everything.
    for entity in &balls_query {
        commands.entity(entity).remove::<Collider>();
    }
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

fn replace_walls_with_paddles(
    walls_query: Query<&Side, With<Wall>>,
    mut spawn_in_goal_events: EventWriter<SpawnEvent<Object, Side>>,
) {
    for side in &walls_query {
        spawn_in_goal_events.send(SpawnEvent::with_data(Object::Paddle, *side));
    }
}

pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameScreen::StartMenu),
            (spawn_start_menu_ui, disable_ball_collisions),
        )
        .add_systems(
            OnExit(GameScreen::StartMenu),
            (reset_each_goals_hit_points, replace_walls_with_paddles).chain(),
        );
    }
}
