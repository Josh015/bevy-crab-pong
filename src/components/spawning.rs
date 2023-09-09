use bevy::prelude::*;

/// Tags an entity to only exist in the listed game states.
#[derive(Component)]
pub struct ForState<T: States> {
    pub states: Vec<T>,
}

/// Marks an entity that needs to be able to spawn in.
#[derive(Bundle, Default)]
pub struct SpawningBundle {
    pub spawning_animation: SpawningAnimation,
    pub spawning: Spawning,
}

/// Specifies an entity's spawning effect animation.
#[derive(Clone, Component, Copy, Default, PartialEq, Debug)]
pub enum SpawningAnimation {
    /// Uses [`StandardMaterial`] color and alpha blending to show/hide entity.
    ///
    /// When paired with [`Fade::In`] the entity's [`StandardMaterial`] must
    /// first be set to [`AlphaMode::Blend`] and have its color alpha set to
    /// zero to avoid visual popping.
    #[default]
    Opacity,

    /// Uses [`Transform`] scale to grow/shrink the entity.
    ///
    /// When paired with [`Fade::In`] the entity's [`Transform`] scale must
    /// first be set to EPSILON to avoid visual popping. We can't use zero
    /// since that prevents it from appearing at all.
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
pub struct Spawning {
    pub progress: f32,
}

/// Marks an entity to fade out and then despawn.
#[derive(Clone, Component, Copy, Default, PartialEq, Debug)]
#[component(storage = "SparseSet")]
pub struct Despawning {
    pub progress: f32,
}
