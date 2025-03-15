use bevy::prelude::*;

use crate::{
    components::{Ball, Collider, Force, Movement, StoppingDistance},
    game::{Goals, PlayableSet},
};

use super::{Crab, CrabCollider};

pub const IDEAL_HIT_AREA_PERCENTAGE: f32 = 0.70;

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

/// Marks a [`Crab`] entity as being controlled by AI.
#[derive(Component, Debug)]
#[require(Crab)]
pub struct AI;

/// The [`Ball`] entity targeted by an [`AI`] [`Crab`] entity.
#[derive(Clone, Component, Debug)]
#[component(storage = "SparseSet")]
pub struct Target(pub Entity);

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
    for (crab_entity, parent, transform, stopping_distance, collider) in
        &crabs_query
    {
        // Target the ball that's closest to the goal.
        let mut closest_ball_distance = f32::MAX;
        let mut closest_ball = None;
        let Ok(goal) = goals.get(parent.get()) else {
            continue;
        };

        for (entity, global_transform) in &balls_query {
            let ball_distance = goal.distance_to(global_transform);

            if ball_distance < closest_ball_distance {
                closest_ball_distance = ball_distance;
                closest_ball = Some((entity, global_transform));
            }
        }

        let target_x = if let Some((entity, global_transform)) = closest_ball {
            commands.entity(crab_entity).insert(Target(entity));
            goal.map_to_local_x(global_transform)
        } else {
            commands.entity(crab_entity).remove::<Target>();
            0.0
        };

        // Move the crab to try to keep its ideal hit area under the ball.
        let crab_x = transform.translation.x;
        let stop_position_x = crab_x + stopping_distance.0;
        let center_distance = (stop_position_x - target_x).abs();

        if center_distance < 0.5 * collider.width * IDEAL_HIT_AREA_PERCENTAGE {
            commands.entity(crab_entity).remove::<Force>();
        } else {
            commands.entity(crab_entity).insert(if target_x < crab_x {
                Force::Negative // Left
            } else {
                Force::Positive // Right
            });
        }
    }
}
