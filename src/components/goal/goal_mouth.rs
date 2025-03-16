use bevy::prelude::*;

use crate::system_sets::StopWhenPausedSet;

use super::{
    Crab, CrabCollider, Force, Goal, Movement, Speed, StoppingDistance,
};

pub(super) struct GoalMouthPlugin;

impl Plugin for GoalMouthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            restrict_crab_movement_to_goal_mouth.after(StopWhenPausedSet),
        );
    }
}

/// Width of the [`Goal`] mouth.
#[derive(Component, Debug, Default)]
#[require(Goal)]
pub struct GoalMouth {
    pub width: f32,
}

fn restrict_crab_movement_to_goal_mouth(
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
