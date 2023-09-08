use crate::{
    cached_assets::CachedAssets,
    components::{balls::Collider, effects::*, fading::*, goals::*},
    constants::*,
    events::*,
    serialization::Config,
    system_sets::GameSystemSet,
};
use bevy::prelude::*;

fn make_camera_slowly_sway_back_and_forth(
    config: Res<Config>,
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<SwayingCamera>, With<Camera3d>)>,
) {
    let mut transform = query.single_mut();
    let x = (time.elapsed_seconds() * config.swaying_camera_speed).sin()
        * GOAL_HALF_WIDTH;

    *transform = Transform::from_xyz(x * 0.5, 2.0, 1.5)
        .looking_at(FIELD_CENTER_POINT, Vec3::Y);
}

fn animate_ocean_with_scrolling_texture_effect(
    config: Res<Config>,
    time: Res<Time>,
    mut query: Query<(&mut Ocean, &mut Transform)>,
) {
    // FIXME: Translate the plane on the Z-axis, since we currently can't
    // animate the texture coordinates.
    let (mut animated_water, mut transform) = query.single_mut();

    *transform = Transform::from_xyz(0.0, -0.01, animated_water.scroll);

    animated_water.scroll += config.animated_water_speed * time.delta_seconds();
    animated_water.scroll %= 1.0;
}

fn handle_spawn_wall_event(
    cached_assets: Res<CachedAssets>,
    mut commands: Commands,
    mut event_reader: EventReader<SpawnWallEvent>,
    goals_query: Query<(Entity, &Side), With<Goal>>,
) {
    for SpawnWallEvent { side, is_instant } in event_reader.iter() {
        for (entity, matching_side) in &goals_query {
            if *side != *matching_side {
                continue;
            }

            commands.entity(entity).with_children(|parent| {
                parent.spawn((
                    *side,
                    Wall,
                    Collider,
                    FadeBundle {
                        fade_animation: FadeAnimation::Scale {
                            max_scale: WALL_SCALE,
                            axis_mask: Vec3::new(0.0, 1.0, 1.0),
                        },
                        fade: Fade::In(if *is_instant { 1.0 } else { 0.0 }),
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
            break;
        }
    }
}

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                make_camera_slowly_sway_back_and_forth,
                animate_ocean_with_scrolling_texture_effect,
                handle_spawn_wall_event,
            )
                .in_set(GameSystemSet::Effects),
        );
    }
}
