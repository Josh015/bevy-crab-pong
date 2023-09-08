use crate::prelude::*;

/// Tags an entity to only exist in the listed game states.
#[derive(Component)]
pub struct ForState<T> {
    pub states: Vec<T>,
}

/// Marks an entity that needs to be able to fade in/out.
#[derive(Bundle, Default)]
pub struct FadeBundle {
    pub fade_animation: FadeAnimation,
    pub fade: Fade,
}

/// Specifies an entity's fade effect animation.
#[derive(Clone, Component, Copy, Default, PartialEq, Debug)]
pub enum FadeAnimation {
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

/// Marks an entity fade in or out and then despawn in the latter case.
#[derive(Clone, Component, Copy, PartialEq, Debug)]
#[component(storage = "SparseSet")]
pub enum Fade {
    /// Fade-in effect with a progress value in the range \[0,1\].
    In(f32),

    /// Fade-out effect with a progress value in the range \[0,1\].
    Out(f32),
}

impl Default for Fade {
    fn default() -> Self {
        Self::In(0.0)
    }
}
