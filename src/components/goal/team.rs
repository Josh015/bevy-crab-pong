use bevy::prelude::*;

use crate::game::{GameState, GoalEliminatedEvent, PlayableSet};

use super::{Goal, HitPoints};

pub(super) struct TeamPlugin;

impl Plugin for TeamPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Last, check_for_winning_team.in_set(PlayableSet));
    }
}

/// Team ID used to check for win conditions based on [`HitPoints`] value.
#[derive(Component, Debug, Default)]
#[require(HitPoints)]
pub struct Team(pub usize);

/// The team that won the previous round.
#[derive(Debug, Default, Resource)]
pub struct WinningTeam(pub usize);

fn check_for_winning_team(
    mut commands: Commands,
    mut goal_eliminated_events: EventReader<GoalEliminatedEvent>,
    mut next_game_state: ResMut<NextState<GameState>>,
    teams_query: Query<(&Team, &HitPoints), With<Goal>>,
) {
    for GoalEliminatedEvent(_) in goal_eliminated_events.read() {
        let mut winning_team = None;
        let survivor = teams_query.iter().find(|(_, hp)| hp.0 > 0);

        if let Some((survivor_team, _)) = survivor {
            let is_winning_team = teams_query
                .iter()
                .all(|(team, hp)| team.0 == survivor_team.0 || hp.0 == 0);

            if is_winning_team {
                winning_team = Some(survivor_team.0);
            }
        } else {
            // Nobody survived. It's a draw!
            winning_team = Some(0);
        }

        if let Some(winning_team) = winning_team {
            commands.insert_resource(WinningTeam(winning_team));
            next_game_state.set(GameState::StartMenu);
            info!("Game Over: Team {winning_team:?} won!");
            break;
        }
    }
}
