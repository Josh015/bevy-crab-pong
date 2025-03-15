use bevy::prelude::*;

use crate::game::{GoalEliminatedEvent, GoalScoredEvent, PlayableSet};

use super::Goal;

pub(super) struct HitPointsPlugin;

impl Plugin for HitPointsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            decrement_hp_when_goal_gets_scored.in_set(PlayableSet),
        );
    }
}

/// How many balls a [`Goal`] can take before it's eliminated.
#[derive(Component, Debug, Default)]
#[require(Goal)]
pub struct HitPoints(pub u8);

fn decrement_hp_when_goal_gets_scored(
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
