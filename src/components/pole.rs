use bevy::prelude::*;

use crate::{system_params::Goals, system_sets::StopWhenPausedSet};

use super::{
    Ball, CircleCollider, Collider, DepthCollider, Direction, Movement,
};

pub(super) struct PolePlugin;

impl Plugin for PolePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            pole_and_ball_collisions.in_set(StopWhenPausedSet),
        );
    }
}

/// Makes an entity a pole that deflects all balls away from a side.
#[derive(Component, Debug)]
pub struct Pole;

fn pole_and_ball_collisions(
    mut commands: Commands,
    goals: Goals,
    poles_query: Query<
        (Entity, &Parent, &DepthCollider),
        (With<Pole>, With<Collider>),
    >,
    balls_query: Query<
        (Entity, &GlobalTransform, &Direction, &CircleCollider),
        (With<Ball>, With<Collider>, With<Movement>),
    >,
) {
    for (pole_entity, parent, depth_collider) in &poles_query {
        let Ok(goal) = goals.get(parent.get()) else {
            continue;
        };

        for (ball_entity, global_transform, direction, collider) in &balls_query
        {
            if !goal.is_facing(direction) {
                continue;
            }

            let ball_distance = goal.distance_to(global_transform);

            if ball_distance > collider.radius + 0.5 * depth_collider.depth {
                continue;
            }

            commands
                .entity(ball_entity)
                .insert(Direction::reflect(direction, -goal.forward()));

            info!("Ball({ball_entity:?}): Collided Pole({pole_entity:?})");
            break;
        }
    }
}
