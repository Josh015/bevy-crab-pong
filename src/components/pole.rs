use bevy::prelude::*;

use crate::{
    components::{
        ball::Ball,
        collider::{CircleCollider, Collider},
        fade::{Fade, FadeEffect, RemoveBeforeFadeOut},
        movement::{Heading, Movement},
        side::{SIDE_WIDTH, Side, SideSpawnPoint},
    },
    game::{assets::CachedAssets, state::PausableSet},
    util::reflect,
};

pub const POLE_DIAMETER: f32 = 0.05;
pub const POLE_HEIGHT: f32 = 0.1;
pub const POLE_RADIUS: f32 = 0.5 * POLE_DIAMETER;

pub(super) struct PolePlugin;

impl Plugin for PolePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_pole_on_side).add_systems(
            PostUpdate,
            pole_and_ball_collisions.in_set(PausableSet),
        );
    }
}

#[derive(Event)]
pub struct SpawnPole {
    pub side: Side,
    pub fade_in: bool,
}

/// Makes an entity a pole that deflects all balls away from a side.
#[derive(Component, Debug)]
pub struct Pole;

fn spawn_pole_on_side(
    trigger: Trigger<SpawnPole>,
    cached_assets: Res<CachedAssets>,
    mut commands: Commands,
    spawn_points_query: Query<(Entity, &Side), With<SideSpawnPoint>>,
) {
    let SpawnPole { side, fade_in } = trigger.event();
    let (spawn_point_entity, _) = spawn_points_query
        .iter()
        .find(|(_, spawn_point_side)| **spawn_point_side == *side)
        .unwrap();

    commands
        .entity(spawn_point_entity)
        .with_children(|builder| {
            builder.spawn((
                Pole,
                *side,
                Collider,
                RemoveBeforeFadeOut::<Collider>::default(),
                if *fade_in {
                    Fade::new_in()
                } else {
                    Fade::In(Timer::default()) // Skip to end of animation.
                },
                FadeEffect::Scale {
                    max_scale: Vec3::new(
                        POLE_DIAMETER,
                        SIDE_WIDTH,
                        POLE_DIAMETER,
                    ),
                    axis_mask: Vec3::new(1.0, 0.0, 1.0),
                },
                Mesh3d(cached_assets.pole_mesh.clone()),
                MeshMaterial3d(cached_assets.pole_material.clone()),
                Transform::from_matrix(Mat4::from_scale_rotation_translation(
                    Vec3::splat(f32::EPSILON),
                    Quat::from_euler(
                        EulerRot::XYZ,
                        0.0,
                        0.0,
                        std::f32::consts::FRAC_PI_2,
                    ),
                    Vec3::new(0.0, POLE_HEIGHT, 0.0),
                )),
            ));
        });

    info!("Pole({side:?}): Spawned");
}

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
