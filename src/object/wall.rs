use bevy::prelude::*;
use spew::prelude::*;

use crate::{
    common::{
        collider::{Collider, ColliderSet},
        fade::{Fade, FadeAnimation, FadeBundle, FADE_DURATION_IN_SECONDS},
        movement::{Heading, Movement},
    },
    game::assets::CachedAssets,
    level::{
        beach::Beach,
        goal::{Goal, GOAL_WIDTH},
        side::Side,
    },
    object::ball::BALL_RADIUS,
    util::reflect,
};

use super::{ball::Ball, Object};

pub const WALL_DIAMETER: f32 = 0.05;
pub const WALL_HEIGHT: f32 = 0.1;
pub const WALL_RADIUS: f32 = 0.5 * WALL_DIAMETER;

/// Makes an entity a wall that deflects all balls away from a goal.
#[derive(Component, Debug)]
pub struct Wall;

pub(super) struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_spawner((Object::Wall, spawn_wall_on_side))
            .add_systems(
                PostUpdate,
                wall_and_ball_collisions.in_set(ColliderSet),
            );
    }
}

fn spawn_wall_on_side(
    In(side): In<Side>,
    cached_assets: Res<CachedAssets>,
    mut commands: Commands,
    beach: Option<Res<Beach>>,
    goals_query: Query<(Entity, &Side), With<Goal>>,
) {
    let (goal_entity, _) = goals_query
        .iter()
        .find(|(_, goal_side)| **goal_side == side)
        .unwrap();

    commands.entity(goal_entity).with_children(|builder| {
        builder.spawn((
            Wall,
            Collider,
            side,
            FadeBundle {
                fade_animation: FadeAnimation::Scale {
                    max_scale: Vec3::new(
                        GOAL_WIDTH,
                        WALL_DIAMETER,
                        WALL_DIAMETER,
                    ),
                    axis_mask: Vec3::new(0.0, 1.0, 1.0),
                },
                fade: Fade::In(Timer::from_seconds(
                    if beach.is_none() {
                        0.0
                    } else {
                        FADE_DURATION_IN_SECONDS
                    },
                    TimerMode::Once,
                )),
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
    });

    info!("Wall({:?}): Spawned", side);
}

fn wall_and_ball_collisions(
    mut commands: Commands,
    balls_query: Query<
        (Entity, &GlobalTransform, &Heading),
        (With<Ball>, With<Collider>, With<Movement>),
    >,
    walls_query: Query<&Side, (With<Wall>, With<Collider>)>,
) {
    for (entity, ball_transform, ball_heading) in &balls_query {
        for side in &walls_query {
            let ball_to_wall_distance = side.distance_to_ball(ball_transform);
            let axis = side.axis();

            // Check that the ball is touching and facing the wall.
            if ball_to_wall_distance > BALL_RADIUS + WALL_RADIUS
                || ball_heading.0.dot(axis) <= 0.0
            {
                continue;
            }

            // Deflect the ball away from the wall.
            commands
                .entity(entity)
                .insert(Heading(reflect(ball_heading.0, axis).normalize()));

            info!("Ball({:?}): Collided Wall({:?})", entity, side);
            break;
        }
    }

    // TODO: Need a fix for the rare occasion when a ball just bounces infinitely
    // between two walls in a straight line? Maybe make all bounces slightly adjust
    // ball angle rather than pure reflection?
}
