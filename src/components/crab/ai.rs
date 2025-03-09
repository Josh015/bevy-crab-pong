use bevy::prelude::*;

use crate::{
    components::{
        ball::Ball,
        collider::Collider,
        movement::{Force, Movement, StoppingDistance},
    },
    game::{state::PlayableSet, system_params::Goals},
};

use super::{Crab, CrabCollider};

pub const AI_CENTER_HIT_AREA_PERCENTAGE: f32 = 0.70;

/// Marks a [`Crab`] entity as being controlled by AI.
#[derive(Component, Debug)]
#[require(Crab)]
pub struct AI;

/// The [`Ball`] entity targeted by an [`AI`] [`Crab`] entity.
#[derive(Clone, Component, Debug)]
#[component(storage = "SparseSet")]
pub struct Target(pub Entity);

pub(super) struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            make_ai_crabs_target_and_move_toward_the_ball_closest_to_their_goal
                .in_set(PlayableSet),
        );
    }
}

fn make_ai_crabs_target_and_move_toward_the_ball_closest_to_their_goal(
    mut commands: Commands,
    goals: Goals,
    crabs_query: Query<
        (
            Entity,
            &Parent,
            &Transform,
            &StoppingDistance,
            &CrabCollider,
        ),
        (With<AI>, With<Crab>, With<Movement>),
    >,
    balls_query: Query<
        (Entity, &GlobalTransform),
        (With<Ball>, With<Movement>, With<Collider>),
    >,
) {
    for (
        crab_entity,
        parent,
        crab_transform,
        stopping_distance,
        crab_collider,
    ) in &crabs_query
    {
        let Ok(goal) = goals.get(parent.get()) else {
            continue;
        };

        let mut closest_ball_distance = f32::MAX;
        let mut closest_ball = None;

        for (ball_entity, ball_global_transform) in &balls_query {
            let ball_distance_to_goal =
                goal.distance_to_ball(ball_global_transform);

            if ball_distance_to_goal < closest_ball_distance {
                closest_ball_distance = ball_distance_to_goal;
                closest_ball = Some((ball_entity, ball_global_transform));
            }
        }

        // Use the ball's side position or default to the center of the side.
        let mut target_goal_position = 0.0;

        if let Some((closest_ball, global_transform)) = closest_ball {
            target_goal_position = goal.map_ball_to_local_x(global_transform);
            commands.entity(crab_entity).insert(Target(closest_ball));
        } else {
            commands.entity(crab_entity).remove::<Target>();
        }

        // Make the crab move to try to keep its ideal hit area under the ball.
        let crab_stop_position =
            crab_transform.translation.x + stopping_distance.0;
        let distance_from_crab_center =
            (crab_stop_position - target_goal_position).abs();

        if distance_from_crab_center
            < 0.5 * crab_collider.width * AI_CENTER_HIT_AREA_PERCENTAGE
        {
            commands.entity(crab_entity).remove::<Force>();
        } else {
            commands.entity(crab_entity).insert(
                if target_goal_position < crab_transform.translation.x {
                    Force::Negative // Left
                } else {
                    Force::Positive // Right
                },
            );
        }
    }
}
