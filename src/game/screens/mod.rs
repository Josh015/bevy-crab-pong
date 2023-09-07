use crate::prelude::*;
use bevy::app::AppExit;

mod paused;
mod start_menu;

pub use paused::*;
pub use start_menu::*;

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

/// Check [`ForState`] entities and either fade out or despawn any that aren't
/// allowed in the current [`AppState`].
fn despawn_invalid_entities_for_state(
    mut commands: Commands,
    game_screen: Res<State<GameScreen>>,
    mut fade_out_entity_events: EventWriter<FadeOutEntityEvent>,
    mut query: Query<(Entity, &ForState<GameScreen>, Option<&FadeAnimation>)>,
) {
    for (entity, for_state, fade_animation) in &mut query {
        if for_state.states.contains(game_screen.get()) {
            continue;
        }

        if fade_animation.is_some() {
            fade_out_entity_events.send(FadeOutEntityEvent(entity));
        } else {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameScreen>()
            .add_systems(Update, game_screen_inputs)
            .add_systems(
                PostUpdate,
                despawn_invalid_entities_for_state
                    .run_if(state_changed::<GameScreen>()),
            )
            .add_plugins((PausedPlugin, StartMenuPlugin));
    }
}