use bevy::prelude::*;

use crate::{
    components::{
        environment::{Ocean, SwayingCamera},
        spawning::{Despawning, SpawnAnimation, SpawnProgress, Spawning},
    },
    constants::*,
    serialization::Config,
};

use super::GameSystemSet;

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

fn start_despawning_entity(
    mut commands: Commands,
    query: Query<(Entity, Option<&Spawning>), Added<Despawning>>,
) {
    for (entity, spawning) in &query {
        if spawning.is_some() {
            commands.entity(entity).remove::<Spawning>();
        }

        info!("Entity({:?}): Despawning", entity);
    }
}

fn advance_spawning_progress(
    mut commands: Commands,
    config: Res<Config>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut SpawnProgress), With<Spawning>>,
) {
    for (entity, mut progress) in &mut query {
        let step = config.fade_speed * time.delta_seconds();

        if progress.0 < FADE_PROGRESS_MAX {
            progress.0 = progress.0.max(FADE_PROGRESS_MIN) + step;
        } else {
            commands.entity(entity).remove::<Spawning>();
            info!("Entity({:?}): Spawned", entity);
        }
    }
}

fn advance_despawning_progress(
    mut commands: Commands,
    config: Res<Config>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut SpawnProgress), With<Despawning>>,
) {
    for (entity, mut progress) in &mut query {
        let step = config.fade_speed * time.delta_seconds();

        if progress.0 > FADE_PROGRESS_MIN {
            progress.0 = progress.0.min(FADE_PROGRESS_MAX) - step;
        } else {
            commands.entity(entity).remove::<Despawning>();
            info!("Entity({:?}): Despawned", entity);
        }
    }
}

fn animate_fade_effect_on_entity(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<
        (
            &mut Transform,
            &Handle<StandardMaterial>,
            &SpawnAnimation,
            &SpawnProgress,
        ),
        Or<(With<Spawning>, With<Despawning>)>,
    >,
) {
    for (mut transform, material, animation, progress) in &mut query {
        match *animation {
            SpawnAnimation::Scale {
                max_scale,
                axis_mask,
            } => {
                transform.scale = (max_scale * axis_mask) * progress.0
                    + (Vec3::ONE - axis_mask);
            },
            SpawnAnimation::Opacity => {
                let material = materials.get_mut(material).unwrap();

                material.base_color.set_a(progress.0);
                material.alpha_mode = if progress.0 < 1.0 {
                    AlphaMode::Blend
                } else {
                    AlphaMode::Opaque
                };
            },
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
                (
                    start_despawning_entity,
                    advance_spawning_progress,
                    advance_despawning_progress,
                    animate_fade_effect_on_entity,
                )
                    .chain(),
            )
                .in_set(GameSystemSet::Effects),
        );
    }
}
