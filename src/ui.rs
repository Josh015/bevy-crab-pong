use crate::prelude::*;
use bevy::app::AppExit;

/// An event fired when spawning a `UiMessage`.
pub struct MessageUiEvent {
    message: String,
    screen: AppState,
}

/// A component for marking a `Text` UI entity as displaying the hit points for
/// an associated `Goal`.
#[derive(Component)]
pub struct HitPointsUi {
    pub goal_side: GoalSide,
}

// TODO: Move UI systems to arena and goal after we make them text meshes?

/// Updates a `Text` entity to display the current life of its associated
/// `Goal`.
pub fn goal_hit_points_ui_system(
    game: Res<RunState>,
    mut query: Query<(&HitPointsUi, &mut Text)>,
) {
    for (hp, mut text) in query.iter_mut() {
        let hit_points = game.goals_hit_points[&hp.goal_side];
        text.sections[0].value = hit_points.to_string();
    }
}

pub fn spawn_ui_message_event(
    run_state: Res<RunState>,
    mut commands: Commands,
    mut event_reader: EventReader<MessageUiEvent>,
) {
    for MessageUiEvent { message, screen } in event_reader.iter() {
        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    ..Default::default()
                },
                color: Color::NONE.into(),
                ..Default::default()
            })
            .insert(ForState {
                states: vec![screen.clone()],
            })
            .with_children(|parent| {
                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(
                                Val::Percent(100.0),
                                Val::Percent(100.0),
                            ),
                            position_type: PositionType::Absolute,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..Default::default()
                        },
                        color: Color::NONE.into(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            style: Style {
                                margin: Rect::all(Val::Px(5.0)),
                                ..Default::default()
                            },
                            text: Text::with_section(
                                message.clone(),
                                TextStyle {
                                    font: run_state.font_handle.clone(),
                                    font_size: 30.0,
                                    color: Color::RED,
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        });
                    });
            });
    }
}

pub fn spawn_start_menu_ui(
    config: Res<GameConfig>,
    run_state: Res<RunState>,
    mut ui_message_events: EventWriter<MessageUiEvent>,
) {
    let mut message = String::new();

    if let Some(game_over) = run_state.game_over {
        message.push_str(match game_over {
            GameOver::Won => &config.game_over_win_message,
            GameOver::Lost => &config.game_over_lose_message,
        });
    }

    message.push_str(&config.new_game_message);

    ui_message_events.send(MessageUiEvent {
        message,
        screen: AppState::StartMenu,
    });
}

pub fn spawn_pause_ui(
    config: Res<GameConfig>,
    mut ui_message_events: EventWriter<MessageUiEvent>,
) {
    ui_message_events.send(MessageUiEvent {
        message: config.pause_message.clone(),
        screen: AppState::Pause,
    });
}

/// Handles all user input regardless of the current game state.
pub fn user_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut state: ResMut<State<AppState>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut query: Query<&mut Movement, (With<Player>, With<Paddle>)>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
    }

    if state.current() != &AppState::StartMenu
        && keyboard_input.just_pressed(KeyCode::Back)
    {
        state.set(AppState::StartMenu).unwrap();
    }

    if state.current() == &AppState::Game {
        // Makes a Paddle entity move left/right in response to the
        // keyboard's corresponding arrows keys.
        for mut movement in query.iter_mut() {
            movement.delta = if keyboard_input.pressed(KeyCode::Left) {
                Some(MovementDelta::Negative)
            } else if keyboard_input.pressed(KeyCode::Right) {
                Some(MovementDelta::Positive)
            } else {
                None
            };
        }

        if keyboard_input.just_pressed(KeyCode::Space) {
            state.set(AppState::Pause).unwrap();
        }
    } else if state.current() == &AppState::Pause {
        if keyboard_input.just_pressed(KeyCode::Space) {
            state.set(AppState::Game).unwrap();
        }
    } else if state.current() == &AppState::StartMenu {
        if keyboard_input.just_pressed(KeyCode::Return) {
            state.set(AppState::Game).unwrap();
        }
    }
}
