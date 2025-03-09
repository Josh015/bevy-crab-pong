use bevy::prelude::*;

use crate::{
    components::{
        ball::Ball,
        collider::{CircleCollider, Collider},
        movement::{Heading, Movement},
    },
    game::{state::PausableSet, system_params::Goals},
    util::reflect,
};

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
    poles_query: Query<&Parent, (With<Pole>, With<Collider>)>,
    balls_query: Query<
        (Entity, &GlobalTransform, &Heading, &CircleCollider),
        (With<Ball>, With<Collider>, With<Movement>),
    >,
) {
    for parent in &poles_query {
        let Ok(goal) = goals.get(parent.get()) else {
            continue;
        };

        for (entity, ball_global_transform, ball_heading, ball_collider) in
            &balls_query
        {
            if !goal.has_incoming_ball(ball_heading) {
                continue;
            }

            let ball_to_pole_distance =
                goal.distance_to_ball(ball_global_transform);

            if ball_to_pole_distance > ball_collider.radius + POLE_RADIUS {
                continue;
            }

            commands.entity(entity).insert(Heading(Dir3::new_unchecked(
                reflect(*ball_heading.0, goal.back).normalize(),
            )));

            info!("Ball({:?}): Collided Pole({:?})", entity, goal.side);
            break;
        }
    }

    // TODO: Need a fix for the rare occasion when a ball just bounces
    // infinitely between two poles in a straight line? Maybe make all
    // bounces slightly adjust ball angle rather than pure reflection?
}
