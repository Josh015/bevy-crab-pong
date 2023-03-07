use crate::prelude::*;

/// An event fired when spawning a message UI.
pub struct MessageUiEvent {
    message: String,
    game_screen: GameScreen,
}

/// A component for marking a [`Text`] UI entity as displaying the hit points
/// for an associated [`Goal`].
#[derive(Component)]
pub struct HitPointsUi;

// TODO: Move UI systems to arena and goal after we make them text meshes?

/// Updates a [`Text`] entity to display the current life of its associated
/// [`Goal`].
fn goal_hit_points_ui(
    game: Res<RunState>,
    mut query: Query<(&Side, &mut Text), With<HitPointsUi>>,
) {
    for (side, mut text) in &mut query {
        let hit_points = game.goals_hit_points[side];
        text.sections[0].value = hit_points.to_string();
    }
}

fn spawn_ui_message_event(
    run_state: Res<RunState>,
    mut commands: Commands,
    mut event_reader: EventReader<MessageUiEvent>,
) {
    for MessageUiEvent {
        message,
        game_screen,
    } in event_reader.iter()
    {
        commands
            .spawn((
                ForState {
                    states: vec![*game_screen],
                },
                NodeBundle {
                    style: Style {
                        size: Size::new(
                            Val::Percent(100.0),
                            Val::Percent(100.0),
                        ),
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    background_color: Color::NONE.into(),
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            size: Size::new(
                                Val::Percent(100.0),
                                Val::Percent(100.0),
                            ),
                            position_type: PositionType::Absolute,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle {
                            style: Style {
                                margin: UiRect::all(Val::Px(5.0)),
                                ..default()
                            },
                            text: Text::from_section(
                                message.clone(),
                                TextStyle {
                                    font: run_state.font_handle.clone(),
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

fn spawn_start_menu_ui(
    config: Res<GameConfig>,
    run_state: Res<RunState>,
    mut ui_message_events: EventWriter<MessageUiEvent>,
) {
    let mut message = String::from(match run_state.game_over {
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

fn spawn_pause_ui(
    config: Res<GameConfig>,
    mut ui_message_events: EventWriter<MessageUiEvent>,
) {
    ui_message_events.send(MessageUiEvent {
        message: config.pause_message.clone(),
        game_screen: GameScreen::Paused,
    });
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MessageUiEvent>().add_systems((
            spawn_start_menu_ui.in_schedule(OnEnter(GameScreen::StartMenu)),
            spawn_pause_ui.in_schedule(OnEnter(GameScreen::Paused)),
            spawn_ui_message_event,
            goal_hit_points_ui,
        ));
    }
}
