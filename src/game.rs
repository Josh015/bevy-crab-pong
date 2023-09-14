use bevy::{prelude::*, utils::HashMap};

use crate::{
    assets::{GameAssets, GameConfig},
    goal::GoalScoredEvent,
    side::Side,
    state::GameState,
};

/// Signals a goal being eliminated from the game.
#[derive(Clone, Component, Debug, Event)]
pub struct CompetitorEliminatedEvent(pub Side);

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
            .add_event::<GoalScoredEvent>()
            .add_systems(OnExit(GameState::Loading), reset_competitors)
            .add_systems(OnExit(GameState::StartMenu), reset_competitors)
            .add_systems(
                PostUpdate,
                (
                    decrement_competitor_hp_when_its_goal_is_scored,
                    check_for_winning_team,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn reset_competitors(
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
                hit_points: u8::from(crab.hit_points),
            },
        );
    }
}

fn decrement_competitor_hp_when_its_goal_is_scored(
    mut goal_scored_events: EventReader<GoalScoredEvent>,
    mut competitor_eliminated_events: EventWriter<CompetitorEliminatedEvent>,
    mut game: ResMut<Game>,
) {
    // Decrement a competitor's HP and potentially eliminate its goal.
    for GoalScoredEvent(side) in goal_scored_events.iter() {
        let Some(competitor) = game.competitors.get_mut(side) else {
            continue;
        };

        competitor.hit_points = competitor.hit_points.saturating_sub(1);

        if competitor.hit_points == 0 {
            competitor_eliminated_events.send(CompetitorEliminatedEvent(*side));
        }
    }
}

fn check_for_winning_team(
    mut next_game_state: ResMut<NextState<GameState>>,
    mut competitor_eliminated_events: EventReader<CompetitorEliminatedEvent>,
    mut game: ResMut<Game>,
) {
    // Check if only one team's competitors still have HP.
    for CompetitorEliminatedEvent(_) in competitor_eliminated_events.iter() {
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
            next_game_state.set(GameState::StartMenu);
            info!("Game Over: Team {:?} won!", winning_team);
            break;
        }
    }
}
