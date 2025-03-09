use bevy::prelude::*;

use crate::game::system_params::Goals;
use crate::game::{events::SideScoredEvent, state::PlayableSet};

use super::{
    ball::Ball,
    collider::{CircleCollider, Collider},
    crab::Crab,
    fade::Fade,
    movement::Movement,
};

pub(super) struct GoalPlugin;

impl Plugin for GoalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            check_if_a_ball_has_scored_in_a_goal.after(PlayableSet),
        );
    }
}

/// Marks a goal that can be used as a parent to spawn entities.
#[derive(Component, Debug)]
#[require(Transform, Visibility)]
pub struct Goal {
    pub width: f32,
}

fn check_if_a_ball_has_scored_in_a_goal(
    mut commands: Commands,
    mut side_scored_events: EventWriter<SideScoredEvent>,
    goals: Goals,
    crabs_query: Query<&Parent, (With<Crab>, With<Movement>, With<Collider>)>,
    balls_query: Query<
        (Entity, &GlobalTransform, &CircleCollider),
        (With<Ball>, With<Movement>, With<Collider>),
    >,
) {
    // If a ball passes a side's alive crab then despawn it and raise an event.
    for parent in &crabs_query {
        let Ok(goal) = goals.get(parent.get()) else {
            continue;
        };

        for (entity, global_transform, collider) in &balls_query {
            let ball_distance = goal.distance_to_ball(global_transform);

            if ball_distance <= collider.radius {
                commands.entity(entity).insert(Fade::new_out());
                side_scored_events.send(SideScoredEvent(goal.side));
                info!("Ball({:?}): Scored Side({:?})", entity, goal.side);
            }
        }
    }
}
