use bevy::prelude::*;
use derive_new::new;

use crate::{
    components::Fade,
    events::GoalScoredEvent,
    system_params::Goals,
    system_sets::{ActiveDuringGameplaySet, StopWhenPausedSet},
};

use super::{
    Ball, CircleCollider, Collider, Crab, Force, Movement, Speed,
    StoppingDistance,
};

pub(super) struct GoalPlugin;

impl Plugin for GoalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            restrict_crab_movement_to_space_within_its_own_goal
                .after(StopWhenPausedSet),
        )
        .add_systems(
            PostUpdate,
            check_if_a_ball_has_scored_in_a_goal
                .in_set(ActiveDuringGameplaySet),
        );
    }
}

/// A goal that contains child entities and can be scored against.
#[derive(Component, Debug, Default)]
#[require(Transform, Visibility)]
pub struct Goal;

/// Specifies the maximum movement area of a [`Goal`] for its child entities.
#[derive(Component, Debug, Default, new)]
#[require(Goal)]
pub struct GoalBounds {
    min: f32,
    max: f32,
}

/// How many balls a [`Goal`] can take before it's eliminated.
#[derive(Component, Debug, Default)]
#[require(Goal)]
pub struct HitPoints(pub u8);

/// Team ID used to check for win conditions based on [`HitPoints`] value.
#[derive(Component, Debug, Default)]
#[require(Goal, HitPoints)]
pub struct Team(pub usize);

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

fn restrict_crab_movement_to_space_within_its_own_goal(
    mut commands: Commands,
    mut crabs_query: Query<
        (
            Entity,
            &Parent,
            &mut Transform,
            &mut Speed,
            &mut StoppingDistance,
        ),
        (With<Crab>, With<Movement>),
    >,
    goals_query: Query<&GoalBounds, With<Goal>>,
) {
    for (entity, parent, mut transform, mut speed, mut stopping_distance) in
        &mut crabs_query
    {
        let Ok(goal_bounds) = goals_query.get(parent.get()) else {
            continue;
        };

        // Limit crab movement to the bounds of its own goal.
        if !(goal_bounds.min..=goal_bounds.max)
            .contains(&transform.translation.x)
        {
            transform.translation.x = transform
                .translation
                .x
                .clamp(goal_bounds.min, goal_bounds.max);
            speed.0 = 0.0;
            commands.entity(entity).remove::<Force>();
        }

        // Also limit stopping distance to the bounds of the goal.
        let stopped_position = transform.translation.x + stopping_distance.0;

        if !(goal_bounds.min..=goal_bounds.max).contains(&stopped_position) {
            stopping_distance.0 = stopped_position.signum() * goal_bounds.max
                - transform.translation.x;
        }
    }
}
