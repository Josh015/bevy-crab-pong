use bevy::prelude::*;

use crate::{
    ball::Ball,
    collider::{Collider, ColliderSet},
    goal::GOAL_WIDTH,
    movement::{Heading, Movement},
    side::Side,
    util::reflect,
};

pub const WALL_DIAMETER: f32 = 0.05;
pub const WALL_HEIGHT: f32 = 0.1;
pub const WALL_RADIUS: f32 = 0.5 * WALL_DIAMETER;
pub const WALL_SCALE: Vec3 =
    Vec3::new(GOAL_WIDTH, WALL_DIAMETER, WALL_DIAMETER);

/// Makes an entity a wall that deflects all balls away from a goal.
#[derive(Component, Debug)]
pub struct Wall;

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            wall_and_ball_collisions.in_set(ColliderSet),
        );
    }
}

fn wall_and_ball_collisions(
    mut commands: Commands,
    balls_query: Query<
        (Entity, &GlobalTransform, &Heading),
        (With<Ball>, With<Movement>, With<Collider>),
    >,
    walls_query: Query<&Side, (With<Wall>, With<Collider>)>,
) {
    for (entity, ball_transform, ball_heading) in &balls_query {
        for side in &walls_query {
            let ball_distance = side.distance_to_ball(ball_transform);
            let axis = side.axis();

            // Check that the ball is touching and facing the wall.
            if ball_distance > WALL_RADIUS || ball_heading.0.dot(axis) <= 0.0 {
                continue;
            }

            // Deflect the ball away from the wall.
            commands
                .entity(entity)
                .insert(Heading(reflect(ball_heading.0, axis)));

            info!("Ball({:?}): Collided Wall({:?})", entity, side);
            break;
        }
    }
}
