use bevy::prelude::*;

use crate::prelude::*;

pub const POLE_DIAMETER: f32 = 0.05;
pub const POLE_HEIGHT: f32 = 0.1;
pub const POLE_RADIUS: f32 = 0.5 * POLE_DIAMETER;

pub(super) struct PolePlugin;

impl Plugin for PolePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            pole_and_ball_collisions.in_set(PausableSet),
        );
    }
}

/// Makes an entity a pole that deflects all balls away from a side.
#[derive(Component, Debug)]
pub struct Pole;

fn pole_and_ball_collisions(
    mut commands: Commands,
    goals: Goals,
    poles_query: Query<(Entity, &Parent), (With<Pole>, With<Collider>)>,
    balls_query: Query<
        (Entity, &GlobalTransform, &Heading, &CircleCollider),
        (With<Ball>, With<Collider>, With<Movement>),
    >,
) {
    for (pole_entity, parent) in &poles_query {
        let Ok(goal) = goals.get(parent.get()) else {
            continue;
        };

        for (ball_entity, global_transform, heading, collider) in &balls_query {
            if !goal.is_facing(heading) {
                continue;
            }

            let ball_distance = goal.distance_to(global_transform);

            if ball_distance > collider.radius + POLE_RADIUS {
                continue;
            }

            commands
                .entity(ball_entity)
                .insert(Heading::reflect(heading, -goal.forward()));

            info!("Ball({ball_entity:?}): Collided Pole({pole_entity:?})");
            break;
        }
    }
}
