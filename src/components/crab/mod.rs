pub mod ai;
pub mod player;

use bevy::prelude::*;

use crate::{
    components::{
        ball::Ball,
        collider::{CircleCollider, Collider},
        movement::{Force, Heading, Movement, Speed, StoppingDistance},
    },
    game::{
        level::{BARRIER_RADIUS, GOAL_WIDTH},
        state::PausableSet,
    },
    util::hemisphere_deflection,
};

use super::goal::Goal;

pub const CRAB_WIDTH: f32 = 0.2;
pub const CRAB_DEPTH: f32 = 0.1;
pub const CRAB_POSITION_X_MAX: f32 =
    (0.5 * GOAL_WIDTH) - BARRIER_RADIUS - (0.5 * CRAB_WIDTH);

pub(super) struct CrabPlugin;

impl Plugin for CrabPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ai::AiPlugin, player::InputPlugin))
            .add_systems(
                Update,
                restrict_crab_movement_to_space_within_its_own_goal
                    .after(PausableSet),
            )
            .add_systems(
                PostUpdate,
                crab_and_ball_collisions.in_set(PausableSet),
            );
    }
}

/// Makes a crab entity that can deflect balls and move sideways inside a goal.
#[derive(Component, Debug, Default)]
pub struct Crab;

/// Physics and collision data for a [Crab] entity.
#[derive(Component, Debug, Default)]
pub struct CrabCollider {
    /// Width of the bounding shape.
    pub width: f32,
}

fn restrict_crab_movement_to_space_within_its_own_goal(
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut Transform, &mut Speed, &mut StoppingDistance),
        (With<Crab>, With<Movement>),
    >,
) {
    for (entity, mut transform, mut speed, mut stopping_distance) in &mut query
    {
        // Limit crab movement to the bounds of its own goal.
        if !(-CRAB_POSITION_X_MAX..=CRAB_POSITION_X_MAX)
            .contains(&transform.translation.x)
        {
            transform.translation.x = transform
                .translation
                .x
                .clamp(-CRAB_POSITION_X_MAX, CRAB_POSITION_X_MAX);
            speed.0 = 0.0;
            commands.entity(entity).remove::<Force>();
        }

        // Also limit stopping distance to the bounds of the goal.
        let stopped_position = transform.translation.x + stopping_distance.0;

        if !(-CRAB_POSITION_X_MAX..=CRAB_POSITION_X_MAX)
            .contains(&stopped_position)
        {
            stopping_distance.0 = stopped_position.signum()
                * CRAB_POSITION_X_MAX
                - transform.translation.x;
        }
    }
}

fn crab_and_ball_collisions(
    mut commands: Commands,
    goals_query: Query<(&Goal, &GlobalTransform)>,
    crabs_query: Query<
        (&Parent, &CrabCollider, &Transform, &GlobalTransform),
        (With<Crab>, With<Collider>),
    >,
    balls_query: Query<
        (Entity, &GlobalTransform, &Heading, &CircleCollider),
        (With<Ball>, With<Collider>, With<Movement>),
    >,
) {
    for (parent, crab_collider, crab_transform, crab_global_transform) in
        &crabs_query
    {
        let Ok((goal, goal_global_transform)) = goals_query.get(parent.get())
        else {
            continue;
        };
        let goal_back = *goal_global_transform.back();
        let crab_right = *crab_global_transform.right();

        for (ball_entity, ball_global_transform, ball_heading, ball_collider) in
            &balls_query
        {
            // Check that the ball is facing the goal.
            if ball_heading.0.dot(goal_back) <= 0.0 {
                continue;
            }

            // Check that the ball is close enough to the goal.
            let ball_to_goal_distance = (0.5 * goal.width)
                - ball_global_transform.translation().dot(goal_back);

            if ball_to_goal_distance > ball_collider.radius + (0.5 * CRAB_DEPTH)
            {
                continue;
            }

            // Check that the ball is close enough to the crab.
            let ball_axis_position =
                ball_global_transform.translation().dot(crab_right);
            let delta = crab_transform.translation.x - ball_axis_position;
            let ball_to_crab_distance = delta.abs();

            if ball_to_crab_distance
                > ball_collider.radius + (0.5 * crab_collider.width)
            {
                continue;
            }

            // Deflect the ball.
            let ball_deflection_direction =
                hemisphere_deflection(delta, crab_collider.width, goal_back);

            commands
                .entity(ball_entity)
                .insert(Heading(Dir3::new_unchecked(
                    ball_deflection_direction.normalize(),
                )));
            info!("Ball({ball_entity:?}): Collided Crab({goal:?})");
            break;
        }
    }
}
