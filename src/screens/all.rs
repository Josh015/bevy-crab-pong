use bevy::{app::AppExit, prelude::*};

use crate::{
    components::spawning::{Despawning, ForStates, SpawnAnimation},
    global_data::GlobalData,
    serialization::Config,
};

use super::GameScreen;

fn handle_game_screen_specific_inputs(
    keyboard_input: Res<Input<KeyCode>>,
    game_screen: Res<State<GameScreen>>,
    config: Res<Config>,
    mut global_data: ResMut<GlobalData>,
    mut next_game_screen: ResMut<NextState<GameScreen>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
        return;
    } else if keyboard_input.just_pressed(KeyCode::G) {
        global_data.is_debugging_enabled = !global_data.is_debugging_enabled;
        return;
    }

    match game_screen.get() {
        GameScreen::StartMenu => {
            if keyboard_input.just_pressed(KeyCode::Return) {
                next_game_screen.set(GameScreen::Playing);
                info!("New Game");
            } else if keyboard_input.just_pressed(KeyCode::Left)
                && global_data.mode_index > 0
            {
                global_data.mode_index -= 1;
                let mode_name = &config.modes[global_data.mode_index].name;
                info!("Game Mode: {mode_name}");
            } else if keyboard_input.just_pressed(KeyCode::Right)
                && global_data.mode_index < config.modes.len() - 1
            {
                global_data.mode_index += 1;
                let mode_name = &config.modes[global_data.mode_index].name;
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

fn despawn_invalid_entities_for_current_screen(
    mut commands: Commands,
    game_screen: Res<State<GameScreen>>,
    mut query: Query<(Entity, &ForStates<GameScreen>, Option<&SpawnAnimation>)>,
) {
    for (entity, for_states, spawning_animation) in &mut query {
        if !for_states.0.contains(game_screen.get()) {
            if spawning_animation.is_some() {
                commands.entity(entity).insert(Despawning);
            } else {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

pub struct AllPlugin;

impl Plugin for AllPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_game_screen_specific_inputs)
            .add_systems(
                PostUpdate,
                despawn_invalid_entities_for_current_screen
                    .run_if(state_changed::<GameScreen>()),
            );
    }
}
