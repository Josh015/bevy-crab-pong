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
    pub width: f32,
}

fn check_if_a_ball_has_scored_in_a_goal(
    mut commands: Commands,
    mut side_scored_events: EventWriter<SideScoredEvent>,
    goals_query: Query<(&Goal, &GlobalTransform, &Side)>,
    crabs_query: Query<&Parent, (With<Crab>, With<Movement>, With<Collider>)>,
    balls_query: Query<
        (Entity, &GlobalTransform, &CircleCollider),
        (With<Ball>, With<Movement>, With<Collider>),
    >,
) {
    // If a ball passes a side's alive crab then despawn it and raise an event.
    for parent in &crabs_query {
        let Ok((goal, goal_global_transform, side)) =
            goals_query.get(parent.get())
        else {
            continue;
        };

        for (ball_entity, ball_global_transform, ball_collider) in &balls_query
        {
            let ball_distance = (0.5 * goal.width)
                - ball_global_transform
                    .translation()
                    .dot(*goal_global_transform.back());

            if ball_distance <= ball_collider.radius {
                commands.entity(ball_entity).insert(Fade::new_out());
                side_scored_events.send(SideScoredEvent(*side));
                info!("Ball({ball_entity:?}): Scored Side({side:?})");
            }
        }
    }
}
