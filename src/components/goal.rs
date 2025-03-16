use bevy::prelude::*;

use crate::{
    components::Fade,
    spawners::SpawnPole,
    states::GameState,
    system_params::Goals,
    system_sets::{ActiveDuringGameplaySet, StopWhenPausedSet},
    ui::WinningTeam,
};

use super::{
    Ball, CircleCollider, Collider, Crab, CrabCollider, Force, Movement, Speed,
    StoppingDistance,
};

pub(super) struct GoalPlugin;

impl Plugin for GoalPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GoalEliminatedEvent>()
            .add_event::<GoalScoredEvent>()
            .add_systems(
                Update,
                restrict_crab_movement_to_space_within_its_own_goal
                    .after(StopWhenPausedSet),
            )
            .add_systems(
                PostUpdate,
                (
                    check_if_a_ball_has_scored_in_a_goal,
                    decrement_hp_and_check_for_eliminated_goals,
                    block_eliminated_goals_and_check_for_winning_team,
                )
                    .chain()
                    .in_set(ActiveDuringGameplaySet),
            );
    }
}

/// A goal that contains child entities and can be scored against.
#[derive(Component, Debug, Default)]
#[require(Transform, Visibility)]
pub struct Goal;

/// Width of the [`Goal`] mouth.
#[derive(Component, Debug, Default)]
#[require(Goal)]
pub struct GoalMouth {
    pub width: f32,
}

/// Team ID used to check for win conditions based on [`HitPoints`] value.
#[derive(Component, Debug, Default)]
#[require(Goal, HitPoints)]
pub struct Team(pub usize);

/// How many balls a [`Goal`] can take before it's eliminated.
#[derive(Component, Debug, Default)]
#[require(Goal)]
pub struct HitPoints(pub u8);

/// Signal when a [`Goal`] entity has been scored by a ball.
#[derive(Clone, Debug, Event)]
struct GoalScoredEvent(pub Entity);

/// Signals that a [`Goal`] has been eliminated from the game.
#[derive(Clone, Debug, Event)]
struct GoalEliminatedEvent(pub Entity);

fn restrict_crab_movement_to_space_within_its_own_goal(
    mut commands: Commands,
    mut crabs_query: Query<
        (
            Entity,
            &Parent,
            &CrabCollider,
            &mut Transform,
            &mut Speed,
            &mut StoppingDistance,
        ),
        (With<Crab>, With<Movement>),
    >,
    goals_query: Query<&GoalMouth, With<Goal>>,
) {
    for (
        entity,
        parent,
        crab_collider,
        mut transform,
        mut speed,
        mut stopping_distance,
    ) in &mut crabs_query
    {
        let Ok(goal_mouth) = goals_query.get(parent.get()) else {
            continue;
        };
        let crab_max_x = 0.5 * (goal_mouth.width - crab_collider.width);

        // Limit crab movement to the bounds of its own goal.
        if !(-crab_max_x..=crab_max_x).contains(&transform.translation.x) {
            transform.translation.x =
                transform.translation.x.clamp(-crab_max_x, crab_max_x);
            speed.0 = 0.0;
            commands.entity(entity).remove::<Force>();
        }

        // Also limit stopping distance to the bounds of the goal.
        let stopped_position = transform.translation.x + stopping_distance.0;

        if !(-crab_max_x..=crab_max_x).contains(&stopped_position) {
            stopping_distance.0 = stopped_position.signum() * crab_max_x
                - transform.translation.x;
        }
    }
}

fn check_if_a_ball_has_scored_in_a_goal(
    mut commands: Commands,
    mut goal_scored_events: EventWriter<GoalScoredEvent>,
    goals: Goals,
    crabs_query: Query<&Parent, (With<Crab>, With<Movement>, With<Collider>)>,
    balls_query: Query<
        (Entity, &GlobalTransform, &CircleCollider),
        (With<Ball>, With<Movement>, With<Collider>),
    >,
) {
    // If a ball passes a side's alive crab then despawn it and raise an event.
    for parent in &crabs_query {
        let goal_entity = parent.get();
        let Ok(goal) = goals.get(goal_entity) else {
            continue;
        };

        for (ball_entity, global_transform, collider) in &balls_query {
            let ball_distance = goal.distance_to(global_transform);

            if ball_distance <= collider.radius {
                commands.entity(ball_entity).insert(Fade::new_out());
                goal_scored_events.send(GoalScoredEvent(goal_entity));
                info!("Ball({ball_entity:?}): Scored Goal({goal_entity:?})");
            }
        }
    }
}

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
