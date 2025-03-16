mod ai;
mod player;

pub use ai::*;
pub use player::*;

use bevy::prelude::*;

use crate::{
    system_params::{GoalData, Goals},
    system_sets::StopWhenPausedSet,
};

use super::{Ball, CircleCollider, Collider, DepthCollider, Heading, Movement};

pub(super) struct CrabPlugin;

impl Plugin for CrabPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AiPlugin, InputPlugin)).add_systems(
            PostUpdate,
            crab_and_ball_collisions.in_set(StopWhenPausedSet),
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

        rotation_away_from_center * goal.forward()
    }
}

fn crab_and_ball_collisions(
    mut commands: Commands,
    goals: Goals,
    crabs_query: Query<
        (Entity, &Parent, &Transform, &CrabCollider, &DepthCollider),
        (With<Crab>, With<Collider>),
    >,
    balls_query: Query<
        (Entity, &GlobalTransform, &Heading, &CircleCollider),
        (With<Ball>, With<Collider>, With<Movement>),
    >,
) {
    for (
        crab_entity,
        parent,
        crab_transform,
        crab_collider,
        crab_depth_collider,
    ) in &crabs_query
    {
        let Ok(goal) = goals.get(parent.get()) else {
            continue;
        };

        for (ball_entity, global_transform, heading, collider) in &balls_query {
            // Check that the ball is facing the goal and close enough to
            // collide.
            if !goal.is_facing(heading) {
                continue;
            }

            let ball_distance = goal.distance_to(global_transform);

            if ball_distance
                > collider.radius + (0.5 * crab_depth_collider.depth)
            {
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
                .entity(ball_entity)
                .insert(Heading::from(new_ball_direction));
            info!("Ball({ball_entity:?}): Collided Crab({crab_entity:?})");
            break;
        }
    }
}
