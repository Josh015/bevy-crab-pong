use bevy::prelude::*;
use spew::prelude::*;

use crate::{
    assets::CachedAssets,
    ball::{Ball, BALL_RADIUS},
    collider::{Collider, ColliderSet},
    goal::GOAL_HALF_WIDTH,
    movement::{Heading, Movement},
    object::Object,
    util::reflect,
};

pub const BARRIER_DIAMETER: f32 = 0.12;
pub const BARRIER_RADIUS: f32 = 0.5 * BARRIER_DIAMETER;
pub const BARRIER_HEIGHT: f32 = 0.2;

/// Marks an entity as a barrier to deflect all balls away from a corner.
#[derive(Component, Debug)]
pub struct Barrier;

pub struct BarrierPlugin;

impl Plugin for BarrierPlugin {
    fn build(&self, app: &mut App) {
        app.add_spawners(((Object::Barrier, spawn_barrier_in_goal),))
            .add_systems(
                PostUpdate,
                barrier_and_ball_collisions.in_set(ColliderSet),
            );
    }
}

fn spawn_barrier_in_goal(
    In(goal_entity): In<Entity>,
    cached_assets: Res<CachedAssets>,
    mut commands: Commands,
) {
    let barrier = commands
        .entity(goal_entity)
        .with_children(|parent| {
            parent.spawn((
                Barrier,
                Collider,
                PbrBundle {
                    mesh: cached_assets.barrier_mesh.clone(),
                    material: cached_assets.barrier_material.clone(),
                    transform: Transform::from_matrix(
                        Mat4::from_scale_rotation_translation(
                            Vec3::new(
                                BARRIER_DIAMETER,
                                BARRIER_HEIGHT,
                                BARRIER_DIAMETER,
                            ),
                            Quat::IDENTITY,
                            Vec3::new(
                                GOAL_HALF_WIDTH,
                                0.5 * BARRIER_HEIGHT,
                                0.0,
                            ),
                        ),
                    ),
                    ..default()
                },
            ));
        })
        .id();

    info!("Barrier({:?}): Spawned", barrier);
}

fn barrier_and_ball_collisions(
    mut commands: Commands,
    balls_query: Query<
        (Entity, &GlobalTransform, &Heading),
        (With<Ball>, With<Movement>, With<Collider>),
    >,
    barriers_query: Query<&GlobalTransform, (With<Barrier>, With<Collider>)>,
) {
    for (entity, ball_transform, ball_heading) in &balls_query {
        for barrier_transform in &barriers_query {
            let ball_to_barrier_distance = ball_transform
                .translation()
                .distance(barrier_transform.translation());

            // Prevent balls from deflecting through the floor.
            let mut axis =
                barrier_transform.translation() - ball_transform.translation();

            axis.y = 0.0;
            axis = axis.normalize();

            // Check that the ball is touching the barrier and facing it.
            if ball_to_barrier_distance > BARRIER_RADIUS + BALL_RADIUS
                || ball_heading.0.dot(axis) <= 0.0
            {
                continue;
            }

            // Deflect the ball away from the barrier.
            commands
                .entity(entity)
                .insert(Heading(reflect(ball_heading.0, axis)));

            info!("Ball({:?}): Collided Barrier", entity);
            break;
        }
    }
}
