use bevy::{prelude::*, utils::HashMap};

use crate::{
    assets::GameAssets, config::GameConfig, goal::GoalEliminatedEvent,
    side::Side, state::AppState,
};

#[derive(Debug, Default)]
pub struct Competitor {
    pub team: usize,
    pub hit_points: u8,
}

/// Global data related to the play area.
#[derive(Debug, Default, Resource)]
pub struct Game {
    pub mode: usize,
    pub competitors: HashMap<Side, Competitor>,
    pub winning_team: Option<usize>,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Game>()
            .add_systems(OnExit(AppState::Loading), reset_teams_and_hit_points)
            .add_systems(
                OnExit(AppState::StartMenu),
                reset_teams_and_hit_points,
            )
            .add_systems(
                PostUpdate,
                check_for_winning_team.run_if(in_state(AppState::Playing)),
            );
    }
}

fn reset_teams_and_hit_points(
    mut game: ResMut<Game>,
    game_assets: Res<GameAssets>,
    game_configs: Res<Assets<GameConfig>>,
) {
    let game_config = game_configs.get(&game_assets.game_config).unwrap();
    let mode = &game_config.modes[game.mode];

    game.competitors.clear();

    for (side, crab) in &mode.competitors {
        game.competitors.insert(
            *side,
            Competitor {
                team: crab.team,
                hit_points: crab.hit_points,
            },
        );
    }
}

fn check_for_winning_team(
    mut next_game_state: ResMut<NextState<AppState>>,
    mut goal_eliminated_events: EventReader<GoalEliminatedEvent>,
    mut game: ResMut<Game>,
) {
    // Check if only one team's competitors still have HP.
    for GoalEliminatedEvent(_) in goal_eliminated_events.iter() {
        let winning_team = {
            let survivor = game
                .competitors
                .iter()
                .find(|(_, competitor)| competitor.hit_points > 0);

            if let Some((_, survivor)) = survivor {
                let is_winner =
                    game.competitors.iter().all(|(_, competitor)| {
                        competitor.team == survivor.team
                            || competitor.hit_points == 0
                    });

                if is_winner {
                    Some(survivor.team)
                } else {
                    None
                }
            } else {
                None
            }
        };

        // Declare a winner and navigate back to the Start Menu.
        if let Some(winning_team) = winning_team {
            game.winning_team = Some(winning_team);
            next_game_state.set(AppState::StartMenu);
            info!("Game Over: Team {:?} won!", winning_team);
            break;
        }
    }
}
