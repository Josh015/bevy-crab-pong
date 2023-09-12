use bevy::{ecs::query::Has, prelude::*};

use crate::state::AppState;

pub const SPAWNING_DURATION_IN_SECONDS: f32 = 1.0;

/// Contains the [`SpawnAnimation`] progress for this entity.
#[derive(Clone, Component, Debug)]
pub struct SpawningProgress(pub Timer);

impl Default for SpawningProgress {
    fn default() -> Self {
        Self(Timer::from_seconds(
            SPAWNING_DURATION_IN_SECONDS,
            TimerMode::Once,
        ))
    }
}

/// Specifies an entity's spawning effect animation.
#[derive(Clone, Component, Copy, Debug, Default, PartialEq)]
pub enum SpawnAnimation {
    /// Uses alpha-blending to fade in/out an entity.
    ///
    /// Will take control of the entity's [`StandardMaterial`] by setting it to
    /// [`AlphaMode::Blend`] and adjusting its `base_color` alpha.
    #[default]
    Opacity,

    /// Uses scale to grow/shrink an entity.
    ///
    /// Will take control of the entity's [`Transform`] `scale`. It must start
    /// with a non-zero scale, or the entity won't appear at all.
    Scale {
        /// The maximum scale to start/end with when fading out/in.
        max_scale: Vec3,

        /// Use either 0/1 to remove/mark an axis for the scale effect.
        axis_mask: Vec3,
    },
}

/// Marks an entity to fade in and delay activation.
#[derive(Clone, Component, Debug, Default)]
#[component(storage = "SparseSet")]
pub struct Spawning;

/// Marks an entity to fade out and then despawn.
#[derive(Clone, Component, Debug, Default)]
#[component(storage = "SparseSet")]
pub struct Despawning;

/// Marks an entity that needs to spawn with animation.
#[derive(Bundle, Clone, Debug, Default)]
pub struct SpawnAnimationBundle {
    pub spawn_animation: SpawnAnimation,
    pub spawning_bundle: SpawningBundle,
}

/// Marks an entity that needs to spawn.
#[derive(Bundle, Clone, Debug, Default)]
pub struct SpawningBundle {
    pub spawning_progress: SpawningProgress,
    pub spawning: Spawning,
}

/// Marks an entity that needs to despawn.
#[derive(Bundle, Clone, Debug, Default)]
pub struct DespawningBundle {
    pub spawning_progress: SpawningProgress,
    pub despawning: Despawning,
}

pub struct SpawningPlugin;

impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                start_despawning_entity,
                spawn_and_finish_animating,
                despawn_when_finished_animating,
                animate_fade_effect_on_entity,
            )
                .chain()
                .run_if(not(in_state(AppState::Paused))),
        )
        .add_systems(Last, finish_despawning_entity);
    }
}

fn start_despawning_entity(
    mut commands: Commands,
    query: Query<(Entity, Has<Spawning>), Added<Despawning>>,
) {
    for (entity, has_spawning) in &query {
        if has_spawning {
            commands.entity(entity).remove::<Spawning>();
        }

        info!("Entity({:?}): Despawning", entity);
    }
}

fn spawn_and_finish_animating(
    mut commands: Commands,
    query: Query<(Entity, &SpawningProgress), With<Spawning>>,
) {
    for (entity, progress) in &query {
        if progress.0.finished() {
            commands.entity(entity).remove::<SpawningBundle>();
            info!("Entity({:?}): Spawned", entity);
        }
    }
}

fn despawn_when_finished_animating(
    mut commands: Commands,
    query: Query<(Entity, &SpawningProgress), With<Despawning>>,
) {
    for (entity, progress) in &query {
        if progress.0.finished() {
            commands.entity(entity).remove::<DespawningBundle>();
            info!("Entity({:?}): Despawned", entity);
        }
    }
}

fn animate_fade_effect_on_entity(
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(
        &mut Transform,
        &Handle<StandardMaterial>,
        &SpawnAnimation,
        &mut SpawningProgress,
        Has<Despawning>,
    )>,
) {
    for (mut transform, material, animation, mut progress, has_despawning) in
        &mut query
    {
        progress.0.tick(time.delta());

        let weight = if has_despawning {
            1.0 - progress.0.percent()
        } else {
            progress.0.percent()
        };

        match *animation {
            SpawnAnimation::Scale {
                max_scale,
                axis_mask,
            } => {
                transform.scale =
                    (max_scale * axis_mask) * weight + (Vec3::ONE - axis_mask);
            },
            SpawnAnimation::Opacity => {
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

fn finish_despawning_entity(
    mut commands: Commands,
    mut removed: RemovedComponents<Despawning>,
) {
    for removed_entity in removed.iter() {
        commands.entity(removed_entity).despawn_recursive();
    }
}
