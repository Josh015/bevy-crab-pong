use bevy::prelude::*;

use crate::{
    components::{Goal, HitPoints, Team},
    spawners::SpawnPole,
    states::GameState,
    system_sets::ActiveDuringGameplaySet,
};

pub(super) struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GoalEliminatedEvent>()
            .add_event::<GoalScoredEvent>()
            .add_systems(
                PostUpdate,
                (
                    decrement_hp_and_check_for_eliminated_goals,
                    block_eliminated_goals_and_check_for_winning_team,
                )
                    .chain()
                    .in_set(ActiveDuringGameplaySet),
            );
    }
}

/// Signal when a [`Goal`] entity has been scored by a ball.
#[derive(Clone, Debug, Event)]
pub struct GoalScoredEvent(pub Entity);

/// Signals that a [`Goal`] has been eliminated from the game.
#[derive(Clone, Debug, Event)]
pub struct GoalEliminatedEvent(pub Entity);

/// The team that won the previous round.
#[derive(Debug, Default, Resource)]
pub struct WinningTeam(pub usize);

fn decrement_hp_and_check_for_eliminated_goals(
    mut goal_scored_events: EventReader<GoalScoredEvent>,
    mut goal_eliminated_events: EventWriter<GoalEliminatedEvent>,
    mut hp_query: Query<&mut HitPoints, With<Goal>>,
) {
    // Decrement a goal's HP and potentially eliminate it.
    for GoalScoredEvent(goal_entity) in goal_scored_events.read() {
        let Ok(mut hp) = hp_query.get_mut(*goal_entity) else {
            continue;
        };

        hp.0 = hp.0.saturating_sub(1);

        if hp.0 == 0 {
            goal_eliminated_events.send(GoalEliminatedEvent(*goal_entity));
            info!("Goal({goal_entity:?}): Eliminated");
        }
    }
}

fn block_eliminated_goals_and_check_for_winning_team(
    mut commands: Commands,
    mut goal_eliminated_events: EventReader<GoalEliminatedEvent>,
    mut next_game_state: ResMut<NextState<GameState>>,
    teams_query: Query<(&Team, &HitPoints), With<Goal>>,
) {
    for GoalEliminatedEvent(goal_entity) in goal_eliminated_events.read() {
        // Block eliminated goals.
        commands.trigger(SpawnPole {
            goal_entity: *goal_entity,
            fade_in: true,
        });

        // Check for a winning team.
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
