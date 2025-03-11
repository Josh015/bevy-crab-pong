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
        system_params::{GoalData, Goals},
    },
};

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

impl CrabCollider {
    /// Get a ball deflection direction based on the its local x delta from
    /// the crab's center.
    pub fn deflect(&self, goal: &GoalData, ball_delta_x: f32) -> Vec3 {
        let rotation_away_from_center = Quat::from_rotation_y(
            std::f32::consts::FRAC_PI_4
                * (ball_delta_x / (0.5 * self.width)).clamp(-1.0, 1.0),
        );

        rotation_away_from_center * goal.forward
    }
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
    goals: Goals,
    crabs_query: Query<
        (&Parent, &Transform, &CrabCollider),
        (With<Crab>, With<Collider>),
    >,
    balls_query: Query<
        (Entity, &GlobalTransform, &Heading, &CircleCollider),
        (With<Ball>, With<Collider>, With<Movement>),
    >,
) {
    for (parent, crab_transform, crab_collider) in &crabs_query {
        let Ok(goal) = goals.get(parent.get()) else {
            continue;
        };

        for (entity, global_transform, heading, collider) in &balls_query {
            // Check that the ball is facing the goal and close enough to
            // collide.
            if !goal.is_facing(heading) {
                continue;
            }

            let ball_distance = goal.distance_to(global_transform);

            if ball_distance > collider.radius + (0.5 * CRAB_DEPTH) {
                continue;
            }

            // Check that the ball is over the crab's hit area.
            let ball_local_x = goal.map_to_local_x(global_transform);
            let ball_delta_x = crab_transform.translation.x - ball_local_x;
            let center_distance = ball_delta_x.abs();

            if center_distance > collider.radius + (0.5 * crab_collider.width) {
                continue;
            }

            // Deflect the ball.
            let new_ball_direction = crab_collider.deflect(&goal, ball_delta_x);

            commands
                .entity(entity)
                .insert(Heading::from(new_ball_direction));
            info!("Ball({:?}): Collided Crab({:?})", entity, goal.side);
            break;
        }
    }
}
