use bevy::prelude::*;

use crate::{
    ball::{Ball, BALL_RADIUS},
    collider::Collider,
    crab::{Crab, CRAB_HALF_WIDTH, CRAB_START_POSITION, CRAB_WIDTH},
    debug_mode::DebugModeSet,
    movement::{Force, Movement, StoppingDistance},
    side::Side,
};

use super::PlayerSet;

pub const AI_CENTER_HIT_AREA_PERCENTAGE: f32 = 0.50;

/// Marks a [`Crab`] entity as being controlled by AI.
#[derive(Component, Debug)]
pub struct PlayerAi;

/// The [`Ball`] entity targeted by an [`AiPlayer`] [`Crab`] entity.
#[derive(Clone, Component, Debug)]
#[component(storage = "SparseSet")]
pub struct Target(pub Entity);

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                make_ai_crabs_target_the_balls_closest_to_their_goals,
                move_ai_crabs_toward_their_targeted_balls,
            )
                .chain()
                .in_set(PlayerSet),
        )
        .add_systems(
            PostUpdate,
            ((
                display_crab_to_ball_targeting_gizmos,
                display_crab_ideal_hit_area_gizmos,
            )
                .in_set(DebugModeSet),),
        );
    }
}

fn make_ai_crabs_target_the_balls_closest_to_their_goals(
    mut commands: Commands,
    crabs_query: Query<
        (Entity, &Side),
        (With<Crab>, With<PlayerAi>, With<Movement>),
    >,
    balls_query: Query<
        (Entity, &GlobalTransform),
        (With<Ball>, With<Movement>, With<Collider>),
    >,
) {
    for (crab_entity, side) in &crabs_query {
        let mut closest_ball_distance = std::f32::MAX;
        let mut target = None;

        for (ball_entity, ball_transform) in &balls_query {
            let ball_distance_to_goal = side.distance_to_ball(ball_transform);

            if ball_distance_to_goal < closest_ball_distance {
                closest_ball_distance = ball_distance_to_goal;
                target = Some(ball_entity);
            }
        }

        if let Some(target) = target {
            commands.entity(crab_entity).insert(Target(target));
        } else {
            commands.entity(crab_entity).remove::<Target>();
        }
    }
}

fn move_ai_crabs_toward_their_targeted_balls(
    mut commands: Commands,
    crabs_query: Query<
        (
            Entity,
            &Side,
            &Transform,
            &StoppingDistance,
            Option<&Target>,
        ),
        (With<Crab>, With<PlayerAi>, With<Movement>),
    >,
    balls_query: Query<
        &GlobalTransform,
        (With<Ball>, With<Movement>, With<Collider>),
    >,
) {
    for (entity, side, transform, stopping_distance, target) in &crabs_query {
        // Use the ball's goal position or default to the center of the goal.
        let mut target_goal_position = CRAB_START_POSITION.x;

        if let Some(target) = target {
            if let Ok(ball_transform) = balls_query.get(target.0) {
                target_goal_position = side.get_ball_position(ball_transform)
            }
        }

        // Move the crab so that its center is over the target goal position.
        let crab_stop_position = transform.translation.x + stopping_distance.0;
        let distance_from_crab_center =
            (crab_stop_position - target_goal_position).abs();

        if distance_from_crab_center
            < BALL_RADIUS + CRAB_HALF_WIDTH * AI_CENTER_HIT_AREA_PERCENTAGE
        {
            commands.entity(entity).remove::<Force>();
        } else {
            commands.entity(entity).insert(
                if target_goal_position < transform.translation.x {
                    Force::Negative // Left
                } else {
                    Force::Positive // Right
                },
            );
        }
    }
}

fn display_crab_to_ball_targeting_gizmos(
    crabs_query: Query<
        (&GlobalTransform, &Target),
        (With<PlayerAi>, With<Crab>, With<Movement>),
    >,
    balls_query: Query<
        &GlobalTransform,
        (With<Ball>, With<Movement>, With<Collider>),
    >,
    mut gizmos: Gizmos,
) {
    for (crab_transform, target) in &crabs_query {
        if let Ok(ball_transform) = balls_query.get(target.0) {
            gizmos.line(
                crab_transform.translation(),
                ball_transform.translation(),
                Color::PURPLE,
            );
        }
    }
}

fn display_crab_ideal_hit_area_gizmos(
    crabs_query: Query<
        &GlobalTransform,
        (With<Crab>, With<PlayerAi>, With<Movement>),
    >,
    mut gizmos: Gizmos,
) {
    for global_transform in &crabs_query {
        let mut hit_area_transform = global_transform.compute_transform();

        hit_area_transform.scale.x = AI_CENTER_HIT_AREA_PERCENTAGE * CRAB_WIDTH;
        gizmos.cuboid(hit_area_transform, Color::YELLOW);
    }
}
