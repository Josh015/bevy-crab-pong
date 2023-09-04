use crate::prelude::*;
use std::collections::HashMap;

/// Current screen of the game.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum GameScreen {
    #[default]
    StartMenu,
    Playing,
    Paused,
}

/// Represents whether the player won or lost the last game.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum GameOver {
    #[default]
    Won,
    Lost,
}

/// All global information for this game.
#[derive(Debug, Resource)]
pub struct RunState {
    pub goals_hit_points: HashMap<Side, u32>,
    pub game_over: Option<GameOver>,
    pub has_debug_gizmos: bool,

    // TODO: Move these to corresponding component files!
    pub font_handle: Handle<Font>,
}

impl FromWorld for RunState {
    fn from_world(world: &mut World) -> Self {
        let font_handle = {
            let asset_server = world.get_resource::<AssetServer>().unwrap();

            asset_server.load("fonts/FiraSans-Bold.ttf")
        };

        Self {
            goals_hit_points: HashMap::with_capacity(4),
            game_over: None,
            font_handle,
            has_debug_gizmos: false,
        }
    }
}

/// Resets all goal HP fields to their starting value.
fn reset_hit_points(config: Res<GameConfig>, mut run_state: ResMut<RunState>) {
    for hit_points in &mut run_state.goals_hit_points.values_mut() {
        *hit_points = config.starting_hit_points;
    }
}

/// Checks for conditions that would trigger a game over.
fn game_over_check(
    mut run_state: ResMut<RunState>,
    mut next_game_screen: ResMut<NextState<GameScreen>>,
    mut event_reader: EventReader<GoalEliminatedEvent>,
    enemies_query: Query<&Side, (With<Paddle>, With<Ai>)>,
    players_query: Query<&Side, (With<Paddle>, With<Player>)>,
) {
    for GoalEliminatedEvent(_) in event_reader.iter() {
        // See if player or enemies have lost enough paddles for a game over.
        let has_player_won = enemies_query
            .iter()
            .all(|side| run_state.goals_hit_points[side] == 0);

        let has_player_lost = players_query
            .iter()
            .all(|side| run_state.goals_hit_points[side] == 0);

        if !has_player_won && !has_player_lost {
            continue;
        }

        // Declare a winner and navigate back to the Start Menu.
        run_state.game_over = Some(if has_player_won {
            GameOver::Won
        } else {
            GameOver::Lost
        });

        next_game_screen.set(GameScreen::StartMenu);
        info!("Game Over: Player {:?}", run_state.game_over.unwrap());
    }
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RunState>()
            .add_state::<GameScreen>()
            .add_systems(OnExit(GameScreen::StartMenu), reset_hit_points)
            .add_systems(
                Update,
                game_over_check.run_if(in_state(GameScreen::Playing)),
            )
            .add_systems(Startup, reset_hit_points);
    }
}
