use bevy::prelude::*;

use crate::{
    components::{
        ball::Ball,
        collider::Collider,
        movement::{Force, Movement, StoppingDistance},
        side::Side,
    },
    game::state::PlayableSet,
};

use super::{CRAB_START_POSITION, CRAB_WIDTH, Crab, CrabWalkAxis};

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
            (
                make_ai_crabs_target_the_ball_closest_to_their_side,
                move_ai_crabs_toward_their_targeted_ball,
            )
                .chain()
                .in_set(PlayableSet),
        );
    }
}

fn make_ai_crabs_target_the_ball_closest_to_their_side(
    mut commands: Commands,
    crabs_query: Query<(Entity, &Side), (With<AI>, With<Crab>, With<Movement>)>,
    balls_query: Query<
        (Entity, &GlobalTransform),
        (With<Ball>, With<Movement>, With<Collider>),
    >,
) {
    for (crab_entity, side) in &crabs_query {
        let mut closest_ball_distance = f32::MAX;
        let mut closest_ball = None;

        for (ball_entity, ball_transform) in &balls_query {
            let ball_distance_to_side = side.distance_to_ball(ball_transform);

            if ball_distance_to_side < closest_ball_distance {
                closest_ball_distance = ball_distance_to_side;
                closest_ball = Some(ball_entity);
            }
        }

        if let Some(closest_ball) = closest_ball {
            commands.entity(crab_entity).insert(Target(closest_ball));
        } else {
            commands.entity(crab_entity).remove::<Target>();
        }
    }
}

fn move_ai_crabs_toward_their_targeted_ball(
    mut commands: Commands,
    crabs_query: Query<
        (
            Entity,
            &Transform,
            &StoppingDistance,
            &CrabWalkAxis,
            Option<&Target>,
        ),
        (With<AI>, With<Crab>, With<Movement>),
    >,
    balls_query: Query<
        &GlobalTransform,
        (With<Ball>, With<Movement>, With<Collider>),
    >,
) {
    for (entity, transform, stopping_distance, walk_axis, target) in
        &crabs_query
    {
        // Use the ball's side position or default to the center of the side.
        let mut target_side_position = CRAB_START_POSITION.x;

        if let Some(target) = target {
            if let Ok(ball_transform) = balls_query.get(target.0) {
                target_side_position =
                    walk_axis.get_axis_position(ball_transform)
            }
        }

        // Make the crab move to try to keep its ideal hit area under the ball.
        let crab_stop_position = transform.translation.x + stopping_distance.0;
        let distance_from_crab_center =
            (crab_stop_position - target_side_position).abs();

        if distance_from_crab_center
            < 0.5 * CRAB_WIDTH * AI_CENTER_HIT_AREA_PERCENTAGE
        {
            commands.entity(entity).remove::<Force>();
        } else {
            commands.entity(entity).insert(
                if target_side_position < transform.translation.x {
                    Force::Negative // Left
                } else {
                    Force::Positive // Right
                },
            );
        }
    }
}
