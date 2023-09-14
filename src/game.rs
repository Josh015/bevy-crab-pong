use bevy::{prelude::*, utils::HashMap};

use crate::{
    assets::{GameAssets, GameConfig},
    goal::{GoalEliminatedEvent, GoalScoredEvent},
    side::Side,
    state::GameState,
};

/// A member of a competing team.
#[derive(Debug, Default)]
pub struct TeamMember {
    pub team: usize,
    pub hit_points: u8,
}

/// The currently selected game mode.
#[derive(Debug, Default, Resource)]
pub struct GameMode(pub usize);

/// All the competitors in the current game.
#[derive(Debug, Default, Resource)]
pub struct Competitors(pub HashMap<Side, TeamMember>);

/// The team that won the previous round.
#[derive(Debug, Default, Resource)]
pub struct WinningTeam(pub usize);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameMode>()
            .init_resource::<Competitors>()
            .add_systems(OnExit(GameState::Loading), reset_competitors)
            .add_systems(OnExit(GameState::StartMenu), reset_competitors)
            .add_systems(
                PostUpdate,
                (
                    decrement_competitor_hp_when_its_goal_is_scored,
                    check_for_game_over_and_winner,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn reset_competitors(
    game_mode: Res<GameMode>,
    game_assets: Res<GameAssets>,
    game_configs: Res<Assets<GameConfig>>,
    mut competitors: ResMut<Competitors>,
) {
    let game_config = game_configs.get(&game_assets.game_config).unwrap();
    let mode = &game_config.modes[game_mode.0];

    competitors.0.clear();

    for (side, competitor) in &mode.competitors {
        competitors.0.insert(
            *side,
            TeamMember {
                team: competitor.team,
                hit_points: u8::from(competitor.hit_points),
            },
        );
    }
}

fn decrement_competitor_hp_when_its_goal_is_scored(
    mut goal_scored_events: EventReader<GoalScoredEvent>,
    mut goal_eliminated_events: EventWriter<GoalEliminatedEvent>,
    mut competitors: ResMut<Competitors>,
) {
    // Decrement a competitor's HP and potentially eliminate its goal.
    for GoalScoredEvent(side) in goal_scored_events.iter() {
        let Some(competitor) = competitors.0.get_mut(side) else {
            continue;
        };

        competitor.hit_points = competitor.hit_points.saturating_sub(1);

        if competitor.hit_points == 0 {
            goal_eliminated_events.send(GoalEliminatedEvent(*side));
        }
    }
}

fn check_for_game_over_and_winner(
    mut commands: Commands,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut goal_eliminated_events: EventReader<GoalEliminatedEvent>,
    competitors: Res<Competitors>,
) {
    // Check if only one team's competitors still have HP.
    for GoalEliminatedEvent(_) in goal_eliminated_events.iter() {
        let Some((_, survivor)) = competitors
            .0
            .iter()
            .find(|(_, competitor)| competitor.hit_points > 0)
        else {
            continue;
        };

        let is_winner = competitors.0.iter().all(|(_, competitor)| {
            competitor.team == survivor.team || competitor.hit_points == 0
        });

        // Declare a winner and navigate back to the Start Menu.
        if is_winner {
            commands.insert_resource(WinningTeam(survivor.team));
            next_game_state.set(GameState::StartMenu);
            info!("Game Over: Team {:?} won!", survivor.team);
            break;
        }
    }
}
