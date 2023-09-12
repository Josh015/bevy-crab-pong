use bevy::{app::AppExit, prelude::*};

use crate::{
    components::{Despawning, ForStates, SpawnAnimation},
    resources::{GameAssets, GameConfig, IsDebuggingMode, SelectedGameMode},
};

use super::GameState;

fn handle_game_state_specific_inputs(
    keyboard_input: Res<Input<KeyCode>>,
    game_state: Res<State<GameState>>,
    game_assets: Res<GameAssets>,
    game_configs: Res<Assets<GameConfig>>,
    mut selected_mode: ResMut<SelectedGameMode>,
    mut is_debugging_mode: ResMut<IsDebuggingMode>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
        return;
    } else if keyboard_input.just_pressed(KeyCode::G) {
        is_debugging_mode.0 = !is_debugging_mode.0;
        return;
    }

    match game_state.get() {
        GameState::StartMenu => {
            let game_config =
                game_configs.get(&game_assets.game_config).unwrap();

            if keyboard_input.just_pressed(KeyCode::Return) {
                next_game_state.set(GameState::Playing);
                info!("New Game");
            } else if keyboard_input.just_pressed(KeyCode::Left)
                && selected_mode.0 > 0
            {
                selected_mode.0 -= 1;
                let mode_name = &game_config.modes[selected_mode.0].name;
                info!("Game Mode: {mode_name}");
            } else if keyboard_input.just_pressed(KeyCode::Right)
                && selected_mode.0 < game_config.modes.len() - 1
            {
                selected_mode.0 += 1;
                let mode_name = &game_config.modes[selected_mode.0].name;
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

fn despawn_invalid_entities_for_state<S: States, const N: usize>(
    mut commands: Commands,
    game_state: Res<State<S>>,
    mut query: Query<(Entity, &ForStates<S, N>, Option<&SpawnAnimation>)>,
) {
    for (entity, for_states, spawning_animation) in &mut query {
        if !for_states.0.contains(game_state.get()) {
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
        app.add_systems(
            Update,
            handle_game_state_specific_inputs
                .run_if(not(in_state(GameState::Loading))),
        )
        .add_systems(
            PostUpdate,
            (
                despawn_invalid_entities_for_state::<GameState, 1>,
                despawn_invalid_entities_for_state::<GameState, 2>,
                despawn_invalid_entities_for_state::<GameState, 3>,
            )
                .run_if(state_changed::<GameState>()),
        );
    }
}