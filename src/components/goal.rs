use bevy::prelude::*;

use crate::{
    components::Fade, events::GoalScoredEvent, system_params::Goals,
    system_sets::PlayableSet,
};

use super::{Ball, CircleCollider, Collider, Crab, Movement};

pub(super) struct GoalPlugin;

impl Plugin for GoalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            check_if_a_ball_has_scored_in_a_goal.in_set(PlayableSet),
        );
    }
}

/// A goal that contains child entities and can be scored against.
#[derive(Component, Debug, Default)]
#[require(Transform, Visibility)]
pub struct Goal;

/// How many balls a [`Goal`] can take before it's eliminated.
#[derive(Component, Debug, Default)]
#[require(Goal)]
pub struct HitPoints(pub u8);

/// Team ID used to check for win conditions based on [`HitPoints`] value.
#[derive(Component, Debug, Default)]
#[require(Goal, HitPoints)]
pub struct Team(pub usize);

/// The team that won the previous round.
#[derive(Debug, Default, Resource)]
pub struct WinningTeam(pub usize);

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
