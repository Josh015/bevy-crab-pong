use bevy::prelude::*;

use crate::{
    components::{
        ball::Ball,
        collider::{CircleCollider, Collider},
        movement::{Heading, Movement},
        side::Side,
    },
    game::state::PausableSet,
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
    balls_query: Query<
        (Entity, &GlobalTransform, &Heading, &CircleCollider),
        (With<Ball>, With<Collider>, With<Movement>),
    >,
    poles_query: Query<&Side, (With<Pole>, With<Collider>)>,
) {
    for (entity, ball_transform, ball_heading, ball_collider) in &balls_query {
        for side in &poles_query {
            let ball_to_pole_distance = side.distance_to_ball(ball_transform);
            let axis = side.axis();

            // Check that the ball is touching and facing the pole.
            if ball_to_pole_distance > ball_collider.radius + POLE_RADIUS
                || ball_heading.0.dot(axis) <= 0.0
            {
                continue;
            }

            // Deflect the ball away from the pole.
            commands.entity(entity).insert(Heading(Dir3::new_unchecked(
                reflect(*ball_heading.0, axis).normalize(),
            )));

            info!("Ball({entity:?}): Collided Pole({side:?})");
            break;
        }
    }

    // TODO: Need a fix for the rare occasion when a ball just bounces
    // infinitely between two poles in a straight line? Maybe make all
    // bounces slightly adjust ball angle rather than pure reflection?
}
