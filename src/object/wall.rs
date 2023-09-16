use bevy::prelude::*;
use spew::prelude::*;

use crate::{
    assets::CachedAssets,
    beach::Beach,
    collider::Collider,
    fade::{Fade, FadeAnimation, FadeBundle, FADE_DURATION_IN_SECONDS},
    goal::{Goal, GOAL_WIDTH},
    object::Object,
    side::Side,
};

pub const WALL_DIAMETER: f32 = 0.05;
pub const WALL_HEIGHT: f32 = 0.1;
pub const WALL_RADIUS: f32 = 0.5 * WALL_DIAMETER;

/// Makes an entity a wall that deflects all balls away from a goal.
#[derive(Component, Debug)]
pub struct Wall;

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_spawner((Object::Wall, spawn_wall_on_side));
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
