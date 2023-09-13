use bevy::prelude::*;

use crate::{
    crab::{Crab, HitPoints},
    goal::GoalEliminatedEvent,
    state::AppState,
};

// An entity's team for checking win conditions.
#[derive(Clone, Component, Debug)]
pub struct Team(pub usize);

/// Indicates which team won the previous round.
#[derive(Debug, Default, Resource)]
pub struct WinningTeam(pub usize);

fn check_for_winning_team(
    mut commands: Commands,
    mut next_game_state: ResMut<NextState<AppState>>,
    mut goal_eliminated_events: EventReader<GoalEliminatedEvent>,
    teams_query: Query<(&Team, &HitPoints), With<Crab>>,
) {
    for GoalEliminatedEvent(_) in goal_eliminated_events.iter() {
        // Check if only one team still has HP.
        let Some((survivor, _)) = teams_query.iter().find(|(_, hp)| hp.0 > 0)
        else {
            return;
        };
        let is_winner = teams_query
            .iter()
            .all(|(team, hp)| team.0 == survivor.0 || hp.0 == 0);

        if !is_winner {
            continue;
        }

        // Declare a winner and navigate back to the Start Menu.
        commands.insert_resource(WinningTeam(survivor.0));
        next_game_state.set(AppState::StartMenu);
        info!("Game Over: Team {:?} won!", survivor.0);
    }
}

pub struct TeamPlugin;

impl Plugin for TeamPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            check_for_winning_team.run_if(in_state(AppState::Playing)),
        );
    }
}
