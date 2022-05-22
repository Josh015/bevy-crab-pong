use crate::prelude::*;

pub const FADE_PROGRESS_MIN: f32 = 0.0;
pub const FADE_PROGRESS_MAX: f32 = 1.0;

#[derive(Bundle, Default)]
pub struct FadeBundle {
    pub fade_animation: FadeAnimation,
    pub fade: Fade,
}

/// A component that specifies the entity's fade effect animation.
#[derive(Clone, Component, Copy, PartialEq, Debug)]
pub enum FadeAnimation {
    /// Uses `StandardMaterial` color and alpha blending to show/hide entity.
    ///
    /// When paired with `Fade::In` the entity's `StandardMaterial` must first
    /// be set to `AlphaMode::Blend` and have its color alpha set to zero to
    /// avoid visual popping.
    Opacity,

    /// Uses `Transform` scale to grow/shrink the entity.
    ///
    /// When paired with `Fade::In` the entity's `Transform` scale must first
    /// be set to EPSILON to avoid visual popping. We can't use zero since that
    /// prevents it from appearing at all.
    Scale {
        /// The maximum scale to start/end with when fading out/in.
        max_scale: Vec3,

        /// Use either 0/1 to remove/mark an axis for the scale effect.
        axis_mask: Vec3,
    },
}

impl Default for FadeAnimation {
    fn default() -> Self {
        Self::Opacity
    }
}

/// A component that makes an entity fade in/out and then despawn if needed.
#[derive(Clone, Component, Copy, PartialEq, Debug)]
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

/// Progresses a `Fade` component to completion before either removing it or
/// despawning the entity.
pub fn fade_system(
    mut commands: Commands,
    config: Res<GameConfig>,
    time: Res<Time>,
    state: Res<State<AppState>>,
    mut query: Query<(Entity, &mut Fade), With<FadeAnimation>>,
) {
    for (entity, mut fade) in query.iter_mut() {
        // Prevent fade animations from running when game is paused.
        if *state.current() == AppState::Pause {
            return;
        }

        // Progress the fade effect.
        let step = config.fade_speed * time.delta_seconds();

        match *fade {
            Fade::In(ref mut progress) => {
                if *progress < FADE_PROGRESS_MAX {
                    *progress = progress.max(FADE_PROGRESS_MIN) + step;
                } else {
                    commands.entity(entity).remove::<Fade>();
                }
            },
            Fade::Out(ref mut progress) => {
                if *progress < FADE_PROGRESS_MAX {
                    *progress = progress.max(FADE_PROGRESS_MIN) + step;
                } else {
                    commands.entity(entity).despawn_recursive();
                }
            },
        }
    }
}

/// Handles `FadeEffect` animations and the transition from
/// visible->invisible and vice versa over time.
pub fn fade_animation_system(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(
        &mut Transform,
        &Handle<StandardMaterial>,
        &FadeAnimation,
        &Fade,
    )>,
) {
    // Apply effect animation to the entity.
    for (mut transform, material, fade_effect, fade) in query.iter_mut() {
        let weight = match *fade {
            Fade::In(progress) => progress,
            Fade::Out(progress) => 1.0 - progress,
        };

        match *fade_effect {
            FadeAnimation::Scale {
                max_scale,
                axis_mask,
            } => {
                transform.scale =
                    (max_scale * axis_mask) * weight + (Vec3::ONE - axis_mask);
            },
            FadeAnimation::Opacity => {
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

/// Makes an entity stop moving & colliding before fading out.
pub fn fade_out_and_stop_entity(commands: &mut Commands, entity: Entity) {
    commands
        .entity(entity)
        .remove::<Movement>()
        .remove::<Collider>()
        .insert(Fade::Out(0.0));
}

/// Makes an entity stop colliding before fading out.
pub fn fade_out_entity(commands: &mut Commands, entity: Entity) {
    commands
        .entity(entity)
        .remove::<Collider>()
        .insert(Fade::Out(0.0));
}
