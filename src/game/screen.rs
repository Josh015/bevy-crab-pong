use crate::prelude::*;
use bevy::app::AppExit;

/// Current screen of the game.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum GameScreen {
    #[default]
    StartMenu,
    Playing,
    Paused,
}

/// Handles inputs specific to each game screen.
fn game_screen_inputs(
    keyboard_input: Res<Input<KeyCode>>,
    game_screen: Res<State<GameScreen>>,
    game_config: Res<GameConfig>,
    mut game_state: ResMut<GameState>,
    mut next_game_screen: ResMut<NextState<GameScreen>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
        return;
    } else if keyboard_input.just_pressed(KeyCode::G) {
        game_state.is_debugging_enabled = !game_state.is_debugging_enabled;
        return;
    }

    match game_screen.get() {
        GameScreen::StartMenu => {
            if keyboard_input.just_pressed(KeyCode::Return) {
                next_game_screen.set(GameScreen::Playing);
                info!("New Game");
            } else if keyboard_input.just_pressed(KeyCode::Left)
                && game_state.mode_index > 0
            {
                game_state.mode_index -= 1;
                let mode_name = &game_config.modes[game_state.mode_index].name;
                info!("Game Mode: {mode_name}");
            } else if keyboard_input.just_pressed(KeyCode::Right)
                && game_state.mode_index < game_config.modes.len() - 1
            {
                game_state.mode_index += 1;
                let mode_name = &game_config.modes[game_state.mode_index].name;
                info!("Game Mode: {mode_name}");
            }
        },
        GameScreen::Playing if keyboard_input.just_pressed(KeyCode::Space) => {
            next_game_screen.set(GameScreen::Paused);
            info!("Paused");
        },
        GameScreen::Paused if keyboard_input.just_pressed(KeyCode::Space) => {
            next_game_screen.set(GameScreen::Playing);
            info!("Unpaused");
        },
        GameScreen::Playing | GameScreen::Paused
            if keyboard_input.just_pressed(KeyCode::Back) =>
        {
            next_game_screen.set(GameScreen::StartMenu);
            info!("Start Menu");
        },
        _ => {},
    }
}

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameScreen>()
            .add_systems(Update, game_screen_inputs);
    }
}
