use crate::prelude::*;
use std::collections::HashMap;

/// Represents whether the player won or lost the last game.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum GameOver {
    #[default]
    Won,
    Lost,
}

/// All global information for this game.
#[derive(Debug, Resource)]
pub struct GameState {
    pub mode_index: usize,
    pub goals_hit_points: HashMap<Side, u32>,
    pub game_over: Option<GameOver>,
    pub is_debugging_enabled: bool,

    // TODO: Move these to corresponding component files!
    pub font_handle: Handle<Font>,
}

impl FromWorld for GameState {
    fn from_world(world: &mut World) -> Self {
        let font_handle = {
            let asset_server = world.get_resource::<AssetServer>().unwrap();

            asset_server.load("fonts/FiraSans-Bold.ttf")
        };

        Self {
            mode_index: 0,
            goals_hit_points: HashMap::with_capacity(4),
            game_over: None,
            font_handle,
            is_debugging_enabled: false,
        }
    }
}

/// Resets all goal HP fields to their starting value.
fn reset_hit_points(
    game_config: Res<GameConfig>,
    mut game_state: ResMut<GameState>,
) {
    const SIDES: [Side; 4] = [Side::Top, Side::Right, Side::Bottom, Side::Left];
    let goals = &game_config.modes[game_state.mode_index].goals;

    for (i, side) in SIDES.iter().enumerate() {
        game_state
            .goals_hit_points
            .insert(*side, goals[i].starting_hit_points);
    }
}

/// Checks for conditions that would trigger a game over.
fn game_over_check(
    mut game_state: ResMut<GameState>,
    mut next_game_screen: ResMut<NextState<GameScreen>>,
    mut event_reader: EventReader<GoalEliminatedEvent>,
    teams_query: Query<(&Team, &Side), With<Paddle>>,
) {
    // TODO: Need a more robust system that allows 4 teams!
    for GoalEliminatedEvent(_) in event_reader.iter() {
        // See if player or enemies have lost enough paddles for a game over.
        let has_player_won = teams_query
            .iter()
            .filter(|(team, _)| **team == Team::Enemies)
            .all(|(_, side)| game_state.goals_hit_points[side] == 0);

        let has_player_lost = teams_query
            .iter()
            .filter(|(team, _)| **team == Team::Allies)
            .all(|(_, side)| game_state.goals_hit_points[side] == 0);

        if !has_player_won && !has_player_lost {
            continue;
        }

        // Declare a winner and navigate back to the Start Menu.
        game_state.game_over = Some(if has_player_won {
            GameOver::Won
        } else {
            GameOver::Lost
        });

        next_game_screen.set(GameScreen::StartMenu);
        info!("Game Over: Player {:?}", game_state.game_over.unwrap());
    }
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameState>()
            .add_systems(OnExit(GameScreen::StartMenu), reset_hit_points)
            .add_systems(
                Update,
                game_over_check.in_set(GameSystemSet::GameplayLogic),
            )
            .add_systems(Startup, reset_hit_points);
    }
}
