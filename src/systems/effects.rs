use bevy::prelude::*;

use crate::{
    components::{
        effects::*,
        spawning::{Despawning, Spawning, SpawningAnimation},
    },
    constants::*,
    serialization::Config,
    systems::GameSystemSet,
};

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
    mut query: Query<
        (Entity, Option<&Spawning>, &mut Despawning),
        Added<Despawning>,
    >,
) {
    for (entity, spawning, mut despawning) in &mut query {
        // If interrupting Spawning then start with its inverse progress to
        // avoid visual popping.
        if let Some(Spawning { progress }) = spawning {
            despawning.progress = 1.0 - progress;
            commands.entity(entity).remove::<Spawning>();
        }

        info!("Entity({:?}): Fading Out", entity);
    }
}

fn advance_spawning_progress(
    mut commands: Commands,
    config: Res<Config>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Spawning)>,
) {
    for (entity, mut spawning) in &mut query {
        let step = config.fade_speed * time.delta_seconds();

        if spawning.progress < FADE_PROGRESS_MAX {
            spawning.progress = spawning.progress.max(FADE_PROGRESS_MIN) + step;
        } else {
            info!("Entity({:?}): Ready", entity);
            commands.entity(entity).remove::<Spawning>();
        }
    }
}

fn advance_despawning_progress(
    mut commands: Commands,
    config: Res<Config>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Despawning)>,
) {
    for (entity, mut despawning) in &mut query {
        // Progress the fade effect.
        let step = config.fade_speed * time.delta_seconds();

        if despawning.progress < FADE_PROGRESS_MAX {
            despawning.progress =
                despawning.progress.max(FADE_PROGRESS_MIN) + step;
        } else {
            info!("Entity({:?}): Despawned", entity);
            commands.entity(entity).remove::<Despawning>();
        }
    }
}

fn animate_fade_effect_on_entity(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<
        (
            &mut Transform,
            &Handle<StandardMaterial>,
            &SpawningAnimation,
            Option<&Spawning>,
            Option<&Despawning>,
        ),
        Or<(With<Spawning>, With<Despawning>)>,
    >,
) {
    // Apply effect animation to the entity.
    for (mut transform, material, spawning_animation, spawning, despawning) in
        &mut query
    {
        let mut weight = 0.0;

        if let Some(Spawning { progress }) = spawning {
            weight = *progress;
        } else if let Some(Despawning { progress }) = despawning {
            weight = 1.0 - *progress;
        }

        match *spawning_animation {
            SpawningAnimation::Scale {
                max_scale,
                axis_mask,
            } => {
                transform.scale =
                    (max_scale * axis_mask) * weight + (Vec3::ONE - axis_mask);
            },
            SpawningAnimation::Opacity => {
                let material = materials.get_mut(material).unwrap();

                material.base_color.set_a(weight);
                material.alpha_mode = if weight < 1.0 {
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
