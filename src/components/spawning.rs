use bevy::prelude::*;

/// Objects that can be spawned via Spew.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Object {
    Ball,
    Wall,
    Paddle,
}

/// Tags an entity to only exist in the listed game states.
#[derive(Clone, Component, Debug, Default)]
pub struct ForStates<T: States>(pub Vec<T>);

/// Contains the [`SpawnAnimation`] progress for this entity.
#[derive(Clone, Component, Debug, Default)]
pub struct SpawnProgress(pub f32);

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
    pub spawning: Spawning,
}
