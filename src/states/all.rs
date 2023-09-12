use bevy::{app::AppExit, ecs::query::Has, prelude::*};

use crate::{
    debug_mode::IsDebuggingMode,
    resources::{GameAssets, GameConfig, SelectedGameMode},
    spawning::{Despawning, SpawnAnimation},
    state::{AppState, ForStates},
};

fn handle_game_state_specific_inputs(
    keyboard_input: Res<Input<KeyCode>>,
    game_state: Res<State<AppState>>,
    game_assets: Res<GameAssets>,
    game_configs: Res<Assets<GameConfig>>,
    mut selected_mode: ResMut<SelectedGameMode>,
    mut is_debugging_mode: ResMut<IsDebuggingMode>,
    mut next_game_state: ResMut<NextState<AppState>>,
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
        AppState::StartMenu => {
            let game_config =
                game_configs.get(&game_assets.game_config).unwrap();

            if keyboard_input.just_pressed(KeyCode::Return) {
                next_game_state.set(AppState::Playing);
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
        AppState::Playing if keyboard_input.just_pressed(KeyCode::Space) => {
            next_game_state.set(AppState::Paused);
            info!("Paused");
        },
        AppState::Paused if keyboard_input.just_pressed(KeyCode::Space) => {
            next_game_state.set(AppState::Playing);
            info!("Unpaused");
        },
        AppState::Playing | AppState::Paused
            if keyboard_input.just_pressed(KeyCode::Back) =>
        {
            next_game_state.set(AppState::StartMenu);
            info!("Start Menu");
        },
        _ => {},
    }
}

fn despawn_invalid_entities_for_state<S: States, const N: usize>(
    mut commands: Commands,
    game_state: Res<State<S>>,
    query: Query<(Entity, &ForStates<S, N>, Has<SpawnAnimation>)>,
) {
    for (entity, for_states, has_spawning_animation) in &query {
        if !for_states.0.contains(game_state.get()) {
            if has_spawning_animation {
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
                .run_if(not(in_state(AppState::Loading))),
        )
        .add_systems(
            PostUpdate,
            (
                despawn_invalid_entities_for_state::<AppState, 1>,
                despawn_invalid_entities_for_state::<AppState, 2>,
                despawn_invalid_entities_for_state::<AppState, 3>,
            )
                .run_if(state_changed::<AppState>()),
        );
    }
}
