use bevy::{prelude::*, window::WindowFocused};
pub use leafwing_input_manager::prelude::*;

use crate::game::{
    assets::{GameAssets, GameConfig},
    competitors::WinningTeam,
    state::{ForStates, GameState, LoadedSet, PlayableSet},
    system_params::GameModes,
};

/// An event fired when spawning a message UI.
#[derive(Debug, Event)]
pub struct MessageUiEvent {
    pub message: String,
    pub game_state: GameState,
}

// List of user actions associated to menu/ui interaction
#[derive(Actionlike, Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect)]
pub enum MenuAction {
    Accept,
    PauseUnpause,
    ReturnToStartMenu,
    NextGameMode,
    PrevGameMode,
    Exit,
}

impl MenuAction {
    fn make_input_map() -> InputMap<Self> {
        use MenuAction::*;

        let mut input_map = InputMap::new([
            (Accept, KeyCode::Enter),
            (PauseUnpause, KeyCode::Space),
            (ReturnToStartMenu, KeyCode::Backspace),
            (PrevGameMode, KeyCode::ArrowLeft),
            (NextGameMode, KeyCode::ArrowRight),
            (Exit, KeyCode::Escape),
        ]);
        input_map.insert_multiple([
            (Accept, GamepadButton::South),
            (PauseUnpause, GamepadButton::Start),
            (ReturnToStartMenu, GamepadButton::Select),
            (PrevGameMode, GamepadButton::DPadLeft),
            (NextGameMode, GamepadButton::DPadRight),
        ]);

        input_map
    }
}

pub(super) struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<MenuAction>::default())
            .init_resource::<ActionState<MenuAction>>()
            .insert_resource(MenuAction::make_input_map())
            .add_event::<MessageUiEvent>()
            .add_systems(OnEnter(GameState::StartMenu), spawn_start_menu_ui)
            .add_systems(OnEnter(GameState::Paused), spawn_pause_ui)
            .add_systems(
                Update,
                (handle_spawn_ui_message_event, handle_menu_inputs)
                    .in_set(LoadedSet),
            )
            .add_systems(
                Update,
                pause_game_when_window_loses_focus.in_set(PlayableSet),
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
    } in message_ui_events.read()
    {
        commands
            .spawn((
                ForStates(vec![*game_state]),
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
            ))
            .with_children(|builder| {
                builder
                    .spawn(Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        position_type: PositionType::Absolute,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    })
                    .with_children(|builder| {
                        builder.spawn((
                            Node {
                                margin: UiRect::all(Val::Px(5.0)),
                                ..default()
                            },
                            Text(message.clone()),
                            TextFont {
                                font: game_assets.font_menu.clone(),
                                font_size: 30.0,
                                ..default()
                            },
                            TextColor(Srgba::RED.into()),
                        ));
                    });
            });
    }
}

fn handle_menu_inputs(
    game_state: Res<State<GameState>>,
    mut game_modes: GameModes,
    mut next_game_state: ResMut<NextState<GameState>>,
    menu_action_state: Res<ActionState<MenuAction>>,
    mut app_exit: EventWriter<AppExit>,
) {
    use GameState::*;
    use MenuAction::*;

    match game_state.get() {
        StartMenu if menu_action_state.just_pressed(&Accept) => {
            next_game_state.set(Playing);
            info!("New Game");
        },
        StartMenu if menu_action_state.just_pressed(&PrevGameMode) => {
            game_modes.previous();
            info!("Game Mode: {}", &game_modes.current().name);
        },
        StartMenu if menu_action_state.just_pressed(&NextGameMode) => {
            game_modes.next();
            info!("Game Mode: {}", &game_modes.current().name);
        },
        Playing if menu_action_state.just_pressed(&PauseUnpause) => {
            next_game_state.set(Paused);
            info!("Paused");
        },
        Paused if menu_action_state.just_pressed(&PauseUnpause) => {
            next_game_state.set(Playing);
            info!("Unpaused");
        },
        Playing | Paused
            if menu_action_state.just_pressed(&ReturnToStartMenu) =>
        {
            next_game_state.set(StartMenu);
            info!("Start Menu");
        },
        _ if menu_action_state.just_pressed(&Exit) => {
            app_exit.send_default();
        },
        _ => {},
    }
}

fn pause_game_when_window_loses_focus(
    mut window_focused_events: EventReader<WindowFocused>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for event in window_focused_events.read() {
        if !event.focused {
            next_game_state.set(GameState::Paused);
            info!("Paused");
        }
    }
}
