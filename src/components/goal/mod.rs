mod goal_mouth;
mod hit_points;
mod team;

pub use goal_mouth::*;
pub use hit_points::*;
pub use team::*;

use bevy::prelude::*;

use crate::{
    components::Fade, spawners::SpawnPole, system_params::Goals,
    system_sets::ActiveDuringGameplaySet,
};

use super::{
    Ball, CircleCollider, Collider, Crab, CrabCollider, Force, Movement, Speed,
    StoppingDistance,
};

pub(super) struct GoalPlugin;

impl Plugin for GoalPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GoalMouthPlugin, HitPointsPlugin, TeamPlugin))
            .add_event::<GoalScoredEvent>()
            .add_event::<GoalEliminatedEvent>()
            .add_systems(
                PostUpdate,
                (check_if_a_ball_has_scored_in_a_goal, block_eliminated_goals)
                    .in_set(ActiveDuringGameplaySet),
            );
    }
}

/// A goal that contains child entities and can be scored against.
#[derive(Component, Debug, Default)]
#[require(Transform, Visibility)]
pub struct Goal;

/// Signal when a [`Goal`] entity has been scored by a ball.
#[derive(Clone, Debug, Event)]
pub(self) struct GoalScoredEvent(pub Entity);

/// Signals that a [`Goal`] has been eliminated from the game.
#[derive(Clone, Debug, Event)]
pub(self) struct GoalEliminatedEvent(pub Entity);

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

fn block_eliminated_goals(
    mut commands: Commands,
    mut goal_eliminated_events: EventReader<GoalEliminatedEvent>,
) {
    for GoalEliminatedEvent(goal_entity) in goal_eliminated_events.read() {
        commands.trigger(SpawnPole {
            goal_entity: *goal_entity,
            fade_in: true,
        });
    }
}
