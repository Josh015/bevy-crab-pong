use std::num::NonZeroUsize;

use bevy::{prelude::*, utils::HashMap};

use crate::level::side::{Side, SideEliminatedEvent, SideScoredEvent};

use super::{
    assets::{GameAssets, GameConfig},
    state::GameState,
    GameSet,
};

/// A member of a competing team.
#[derive(Debug)]
pub struct TeamMember {
    pub team: NonZeroUsize,
    pub hit_points: u8,
}

/// The currently selected game mode.
#[derive(Debug, Default, Resource)]
pub struct GameMode(pub usize);

/// All the competitors in the current round.
#[derive(Debug, Default, Resource)]
pub struct Competitors(pub HashMap<Side, TeamMember>);

/// The team that won the previous round.
#[derive(Debug, Default, Resource)]
pub struct WinningTeam(pub usize);

pub(super) struct CompetitorsPlugin;

impl Plugin for CompetitorsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameMode>()
            .init_resource::<Competitors>()
            .add_systems(OnExit(GameState::Loading), reset_competitors)
            .add_systems(OnExit(GameState::StartMenu), reset_competitors)
            .add_systems(
                PostUpdate,
                (
                    decrement_competitor_hp_when_their_side_gets_scored,
                    check_for_game_over,
                )
                    .chain()
                    .in_set(GameSet),
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

fn decrement_competitor_hp_when_their_side_gets_scored(
    mut side_scored_events: EventReader<SideScoredEvent>,
    mut side_eliminated_events: EventWriter<SideEliminatedEvent>,
    mut competitors: ResMut<Competitors>,
) {
    // Decrement a competitor's HP and potentially eliminate their side.
    for SideScoredEvent(side) in side_scored_events.read() {
        let competitor = competitors.0.get_mut(side).unwrap();

        competitor.hit_points = competitor.hit_points.saturating_sub(1);

        if competitor.hit_points == 0 {
            side_eliminated_events.send(SideEliminatedEvent(*side));
        }
    }
}

fn check_for_game_over(
    mut commands: Commands,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut side_eliminated_events: EventReader<SideEliminatedEvent>,
    competitors: Res<Competitors>,
) {
    for SideEliminatedEvent(_) in side_eliminated_events.read() {
        let mut winning_team = None;
        let survivor = competitors
            .0
            .iter()
            .find(|(_, competitor)| competitor.hit_points > 0);

        if let Some((_, survivor)) = survivor {
            let is_winner = competitors.0.iter().all(|(_, competitor)| {
                competitor.team == survivor.team || competitor.hit_points == 0
            });

            if is_winner {
                winning_team = Some(usize::from(survivor.team));
            }
        } else {
            // Nobody survived. It's a draw!
            winning_team = Some(0);
        }

        if let Some(winning_team) = winning_team {
            commands.insert_resource(WinningTeam(winning_team));
            next_game_state.set(GameState::StartMenu);
            info!("Game Over: Team {:?} won!", winning_team);
            break;
        }
    }
}
