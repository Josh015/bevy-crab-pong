use bevy::prelude::*;
use spew::prelude::{SpawnEvent, SpewSystemSet};

use crate::{
    ball::{Ball, BALL_HEIGHT},
    object::Object,
    resources::{GameAssets, GameConfig, SelectedGameMode},
    spawning::Spawning,
    state::AppState,
};

pub const ARENA_CENTER_POINT: Vec3 = Vec3::ZERO;
pub const ARENA_BALL_SPAWNER_POSITION: Vec3 = Vec3::new(
    ARENA_CENTER_POINT.x,
    ARENA_CENTER_POINT.y + BALL_HEIGHT,
    ARENA_CENTER_POINT.z,
);

/// Global data related to the play area.
#[derive(Debug, Default, Resource)]
pub struct Arena {
    max_ball_count: u8,
}

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::StartMenu), initialize_arena_data)
            .add_systems(
                Update,
                spawn_balls_sequentially_as_needed
                    .before(SpewSystemSet)
                    .run_if(in_state(AppState::Playing)),
            );
    }
}

fn initialize_arena_data(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    game_configs: Res<Assets<GameConfig>>,
    selected_mode: Res<SelectedGameMode>,
) {
    let game_config = game_configs.get(&game_assets.game_config).unwrap();

    commands.insert_resource(Arena {
        max_ball_count: game_config.modes[selected_mode.0].max_ball_count,
    })
}

fn spawn_balls_sequentially_as_needed(
    arena: Res<Arena>,
    balls_query: Query<Entity, With<Ball>>,
    spawning_balls_query: Query<Entity, (With<Ball>, With<Spawning>)>,
    mut spawn_events: EventWriter<SpawnEvent<Object>>,
) {
    if balls_query.iter().len() < arena.max_ball_count as usize
        && spawning_balls_query.iter().len() < 1
    {
        spawn_events.send(SpawnEvent::new(Object::Ball));
    }
}
