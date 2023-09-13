use bevy::prelude::*;
use spew::prelude::*;

use crate::{
    assets::CachedAssets,
    ball::Ball,
    collider::{Collider, ColliderSet},
    fade::{FadeAnimation, FadeBundle},
    goal::{Goal, GOAL_WIDTH},
    movement::{Heading, Movement},
    object::Object,
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
        app.add_spawners(((Object::Wall, spawn_wall_in_goal),))
            .add_systems(
                PostUpdate,
                wall_and_crab_collisions.in_set(ColliderSet),
            );
    }
}

fn spawn_wall_in_goal(
    In(goal_entity): In<Entity>,
    cached_assets: Res<CachedAssets>,
    mut commands: Commands,
    goals_query: Query<&Side, With<Goal>>,
) {
    let Ok(goal_side) = goals_query.get(goal_entity) else {
        return;
    };

    // Spawn wall in goal.
    let wall = commands
        .entity(goal_entity)
        .with_children(|parent| {
            parent.spawn((
                Wall,
                Collider,
                *goal_side,
                FadeBundle {
                    fade_animation: FadeAnimation::Scale {
                        max_scale: WALL_SCALE,
                        axis_mask: Vec3::new(0.0, 1.0, 1.0),
                    },
                    ..default()
                },
                PbrBundle {
                    mesh: cached_assets.wall_mesh.clone(),
                    material: cached_assets.wall_material.clone(),
                    transform: Transform::from_matrix(
                        Mat4::from_scale_rotation_translation(
                            Vec3::splat(f32::EPSILON),
                            Quat::IDENTITY,
                            Vec3::new(0.0, WALL_HEIGHT, 0.0),
                        ),
                    ),
                    ..default()
                },
            ));
        })
        .id();

    info!("Wall({:?}): Spawned", wall);
}

fn wall_and_crab_collisions(
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
