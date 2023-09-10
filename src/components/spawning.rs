use bevy::prelude::*;

/// Objects that can be spawned via Spew.
#[derive(Debug, Eq, PartialEq)]
pub enum Object {
    Ball,
    Wall,
    Paddle,
}

/// Tags an entity to only exist in the listed game states.
#[derive(Component)]
pub struct ForState<T: States> {
    pub states: Vec<T>,
}

/// Contains the [`SpawnAnimation`] progress for this entity.
#[derive(Component, Default)]
pub struct SpawnProgress(pub f32);

/// Specifies an entity's spawning effect animation.
#[derive(Clone, Component, Copy, Default, PartialEq, Debug)]
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
#[derive(Clone, Component, Copy, Default, PartialEq, Debug)]
#[component(storage = "SparseSet")]
pub struct Spawning;

/// Marks an entity to fade out and then despawn.
#[derive(Clone, Component, Copy, Default, PartialEq, Debug)]
#[component(storage = "SparseSet")]
pub struct Despawning;

/// Marks an entity that needs spawn effects.
#[derive(Bundle, Default)]
pub struct SpawnEffectsBundle {
    pub spawn_animation: SpawnAnimation,
    pub spawn_progress: SpawnProgress,
    pub spawning: Spawning,
}
