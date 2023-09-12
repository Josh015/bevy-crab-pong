use bevy::{ecs::query::Has, prelude::*};

pub const FADE_PROGRESS_MIN: f32 = 0.0;
pub const FADE_PROGRESS_MAX: f32 = 1.0;

/// Contains the [`SpawnAnimation`] progress for this entity.
#[derive(Clone, Component, Debug, Default)]
pub struct SpawnProgress(pub f32);

/// Contains the [`SpawnAnimation`] playback speed.
#[derive(Clone, Component, Debug, Default)]
pub struct SpawnSpeed(pub f32);

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
#[derive(Component, Debug)]
#[component(storage = "SparseSet")]
pub struct Despawning;

/// Marks an entity that needs spawn effects.
#[derive(Bundle, Clone, Debug, Default)]
pub struct SpawnEffectsBundle {
    pub spawn_animation: SpawnAnimation,
    pub spawn_progress: SpawnProgress,
    pub spawn_speed: SpawnSpeed,
    pub spawning: Spawning,
}

pub struct SpawningPlugin;

impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                start_despawning_entity,
                advance_spawning_progress,
                advance_despawning_progress,
                animate_fade_effect_on_entity,
            )
                .chain(),
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

fn advance_spawning_progress(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut SpawnProgress, &SpawnSpeed), With<Spawning>>,
) {
    for (entity, mut progress, spawn_speed) in &mut query {
        let step = spawn_speed.0 * time.delta_seconds();

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
    time: Res<Time>,
    mut query: Query<
        (Entity, &mut SpawnProgress, &SpawnSpeed),
        With<Despawning>,
    >,
) {
    for (entity, mut progress, spawn_speed) in &mut query {
        let step = spawn_speed.0 * time.delta_seconds();

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

fn finish_despawning_entity(
    mut commands: Commands,
    mut removed: RemovedComponents<Despawning>,
) {
    for removed_entity in removed.iter() {
        commands.entity(removed_entity).despawn_recursive();
    }
}
