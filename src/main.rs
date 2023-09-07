mod components;
mod config;
mod files;
mod state;

pub mod prelude {
    pub use crate::{components::*, config::*, state::*};
    pub use bevy::{math::*, prelude::*};
    pub use rand::prelude::*;
}

use crate::prelude::*;
use bevy::{
    app::AppExit,
    window::{PresentMode, WindowResolution},
};

fn main() {
    let config: GameConfig =
        files::load_config_from_file("assets/config/game.ron");

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: config.title.clone(),
                resolution: WindowResolution::new(
                    config.width as f32,
                    config.height as f32,
                ),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Msaa::default())
        .insert_resource(ClearColor(config.clear_color))
        .insert_resource(config)
        .add_plugins((StatePlugin, ComponentsPlugin))
        .add_systems(Update, input)
        .run();
}

/// Handles all user input regardless of the current game state.
fn input(
    keyboard_input: Res<Input<KeyCode>>,
    game_screen: Res<State<GameScreen>>,
    mut run_state: ResMut<RunState>,
    config: Res<GameConfig>,
    mut next_game_screen: ResMut<NextState<GameScreen>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
        return;
    } else if keyboard_input.just_pressed(KeyCode::G) {
        run_state.is_debugging_enabled = !run_state.is_debugging_enabled;
        return;
    }

    match game_screen.get() {
        GameScreen::StartMenu => {
            if keyboard_input.just_pressed(KeyCode::Return) {
                next_game_screen.set(GameScreen::Playing);
                info!("New Game");
            } else if keyboard_input.just_pressed(KeyCode::Left)
                && run_state.mode_index > 0
            {
                run_state.mode_index -= 1;
                let mode_name = &config.modes[run_state.mode_index].name;
                info!("Game Mode: {mode_name}");
            } else if keyboard_input.just_pressed(KeyCode::Right)
                && run_state.mode_index < config.modes.len() - 1
            {
                run_state.mode_index += 1;
                let mode_name = &config.modes[run_state.mode_index].name;
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

// TODO: Need a fix for the rare occasion when a ball just bounces infinitely
// between two walls in a straight line? Maybe make all bounces slightly adjust
// ball angle rather than pure reflection?

// TODO: Offer a "Traditional" mode with two paddles (1xPlayer, 1xAi)
// opposite each other and the other two walled off. Also just one ball?

// TODO: Debug option to directly control single ball's exact position with
// keyboard and see how paddles respond. Can go in goals, triggering a score and
// ball return?
