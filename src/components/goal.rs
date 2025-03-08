use bevy::prelude::*;

use crate::components::side::Side;

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
    pub forward: Vec3,
    pub width: f32,
}

impl Goal {
    /// Perpendicular distance from a given goal to an entity's center.
    ///
    /// Positive distances for inside, negative for out of bounds.
    pub fn distance_to_entity(
        &self,
        global_transform: &GlobalTransform,
    ) -> f32 {
        (0.5 * self.width) - global_transform.translation().dot(self.forward)
    }
}

fn check_if_a_ball_has_scored_in_a_goal(
    mut commands: Commands,
    mut side_scored_events: EventWriter<SideScoredEvent>,
    goals_query: Query<(&Goal, &Side)>,
    crabs_query: Query<&Parent, (With<Crab>, With<Movement>, With<Collider>)>,
    balls_query: Query<
        (Entity, &GlobalTransform, &CircleCollider),
        (With<Ball>, With<Movement>, With<Collider>),
    >,
) {
    // If a ball passes a side's alive crab then despawn it and raise an event.
    for parent in &crabs_query {
        let Ok((goal, side)) = goals_query.get(parent.get()) else {
            continue;
        };

        for (ball_entity, ball_global_transform, ball_collider) in &balls_query
        {
            let ball_distance = goal.distance_to_entity(ball_global_transform);

            if ball_distance <= ball_collider.radius {
                commands.entity(ball_entity).insert(Fade::new_out());
                side_scored_events.send(SideScoredEvent(*side));
                info!("Ball({ball_entity:?}): Scored Side({side:?})");
            }
        }
    }
}
