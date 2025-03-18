use bevy::{prelude::*, window::WindowFocused};
use leafwing_input_manager::prelude::*;
use rust_i18n::t;

use crate::{
    assets::{GameAssets, GameConfig},
    components::Player,
    spawners::SpawnUiMessage,
    states::GameState,
    system_params::GameModes,
    system_sets::{ActiveAfterLoadingSet, ActiveDuringGameplaySet},
};

pub(super) struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<MenuAction>::default())
            .init_resource::<ActionState<MenuAction>>()
            .insert_resource(MenuAction::make_input_map())
            .add_systems(OnEnter(GameState::StartMenu), show_start_menu_ui)
            .add_systems(OnEnter(GameState::Paused), show_pause_ui)
            .add_systems(
                Update,
                handle_menu_inputs.in_set(ActiveAfterLoadingSet),
            )
            .add_systems(
                Update,
                pause_player_controlled_game_when_window_loses_focus
                    .in_set(ActiveDuringGameplaySet),
            );
    }
}

/// The team that won the previous round.
#[derive(Debug, Default, Resource)]
pub struct WinningTeam(pub usize);

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

fn show_start_menu_ui(
    mut commands: Commands,
    winning_team: Option<Res<WinningTeam>>,
    game_assets: Res<GameAssets>,
    game_configs: Res<Assets<GameConfig>>,
) {
    let game_config = game_configs.get(&game_assets.game_config).unwrap();
    let mut message = String::from(match winning_team {
        Some(winning_team) => {
            let winning_message =
                game_config.winning_team_messages[winning_team.0].as_str();
            t!(winning_message).to_string()
        },
        _ => "".to_string(),
    });

    message.push_str(&t!("ui.start_menu.new_game"));

    commands.trigger(SpawnUiMessage {
        message,
        game_state: GameState::StartMenu,
    });
}

fn show_pause_ui(mut commands: Commands) {
    commands.trigger(SpawnUiMessage {
        message: t!("ui.pause_menu.paused").to_string(),
        game_state: GameState::Paused,
    });
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

fn pause_player_controlled_game_when_window_loses_focus(
    mut window_focused_events: EventReader<WindowFocused>,
    mut next_game_state: ResMut<NextState<GameState>>,
    players_query: Query<Entity, With<Player>>,
) {
    for event in window_focused_events.read() {
        if !event.focused && !players_query.is_empty() {
            next_game_state.set(GameState::Paused);
            info!("Paused");
        }
    }
}
