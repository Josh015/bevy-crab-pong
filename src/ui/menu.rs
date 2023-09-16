use bevy::prelude::*;

use crate::game::{
    assets::{GameAssets, GameConfig},
    competitors::{GameMode, WinningTeam},
    state::{ForStates, GameState, LoadedSet},
};

/// An event fired when spawning a message UI.
#[derive(Event, Debug)]
pub struct MessageUiEvent {
    pub message: String,
    pub game_state: GameState,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MessageUiEvent>()
            .add_systems(OnEnter(GameState::StartMenu), spawn_start_menu_ui)
            .add_systems(OnEnter(GameState::Paused), spawn_pause_ui)
            .add_systems(
                Update,
                (handle_spawn_ui_message_event, handle_menu_inputs)
                    .in_set(LoadedSet),
            );
    }
}

fn spawn_start_menu_ui(
    game_assets: Res<GameAssets>,
    game_configs: Res<Assets<GameConfig>>,
    winning_team: Option<Res<WinningTeam>>,
    mut ui_message_events: EventWriter<MessageUiEvent>,
) {
    let game_config = game_configs.get(&game_assets.game_config).unwrap();
    let mut message = String::from(match winning_team {
        Some(winning_team) => &game_config.team_win_messages[winning_team.0],
        _ => "",
    });

    message.push_str(&game_config.new_game_message);

    ui_message_events.send(MessageUiEvent {
        message,
        game_state: GameState::StartMenu,
    });
}

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

fn handle_spawn_ui_message_event(
    game_assets: Res<GameAssets>,
    mut commands: Commands,
    mut message_ui_events: EventReader<MessageUiEvent>,
) {
    for MessageUiEvent {
        message,
        game_state,
    } in message_ui_events.iter()
    {
        commands
            .spawn((
                ForStates(vec![*game_state]),
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    background_color: Color::NONE.into(),
                    ..default()
                },
            ))
            .with_children(|builder| {
                builder
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            position_type: PositionType::Absolute,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    })
                    .with_children(|builder| {
                        builder.spawn(TextBundle {
                            style: Style {
                                margin: UiRect::all(Val::Px(5.0)),
                                ..default()
                            },
                            text: Text::from_section(
                                message.clone(),
                                TextStyle {
                                    font: game_assets.font_menu.clone(),
                                    font_size: 30.0,
                                    color: Color::RED,
                                },
                            ),
                            ..default()
                        });
                    });
            });
    }
}

fn handle_menu_inputs(
    keyboard_input: Res<Input<KeyCode>>,
    game_state: Res<State<GameState>>,
    game_assets: Res<GameAssets>,
    game_configs: Res<Assets<GameConfig>>,
    mut game_mode: ResMut<GameMode>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    match game_state.get() {
        GameState::StartMenu => {
            let game_config =
                game_configs.get(&game_assets.game_config).unwrap();

            if keyboard_input.just_pressed(KeyCode::Return) {
                next_game_state.set(GameState::Playing);
                info!("New Game");
            } else if keyboard_input.just_pressed(KeyCode::Left)
                && game_mode.0 > 0
            {
                game_mode.0 -= 1;
                let mode_name = &game_config.modes[game_mode.0].name;
                info!("Game Mode: {mode_name}");
            } else if keyboard_input.just_pressed(KeyCode::Right)
                && game_mode.0 < game_config.modes.len() - 1
            {
                game_mode.0 += 1;
                let mode_name = &game_config.modes[game_mode.0].name;
                info!("Game Mode: {mode_name}");
            }
        },
        GameState::Playing if keyboard_input.just_pressed(KeyCode::Space) => {
            next_game_state.set(GameState::Paused);
            info!("Paused");
        },
        GameState::Paused if keyboard_input.just_pressed(KeyCode::Space) => {
            next_game_state.set(GameState::Playing);
            info!("Unpaused");
        },
        GameState::Playing | GameState::Paused
            if keyboard_input.just_pressed(KeyCode::Back) =>
        {
            next_game_state.set(GameState::StartMenu);
            info!("Start Menu");
        },
        _ => {},
    }
}
