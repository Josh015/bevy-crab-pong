use crate::prelude::*;

pub const MIN_FADE_PROGRESS: f32 = 0.0;
pub const MAX_FADE_PROGRESS: f32 = 1.0;

/// A component that specifies the entity's fade effect animation.
#[derive(Clone, Component, Copy, PartialEq, Debug)]
pub enum FadeAnimation {
    /// Effect that uses material opacity and alpha blending.
    ///
    /// When paired with `Fade::In` the entity's `StandardMaterial` must first
    /// be set to `AlphaMode::Blend` and have its color alpha set to zero to
    /// avoid visual popping.
    Translucency,

    /// Effect that controls the transform scale of the entity.
    ///
    /// When paired with `Fade::In` the entity's `Transform` scale must first
    /// be set to EPSILON to avoid visual popping. We can't use zero since that
    /// prevents it from appearing at all.
    Scaling {
        /// The maximum scale to start/end with when fading out/in.
        max_scale: Vec3,

        /// Use either 0/1 to remove/mark an axis for the scale effect.
        axis_mask: Vec3,
    },
}

/// A component that makes an entity fade in/out and then despawn if needed.
#[derive(Clone, Component, Copy, PartialEq, Debug)]
pub enum Fade {
    /// Applies a fade-in effect via a progress value \[0,1\].
    In(f32),

    /// Applies a fade-out effect via a progress value \[0,1\].
    Out(f32),
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
    // Prevent fade animations from running when game is paused.
    if *state.current() == AppState::Pause {
        return;
    }

    // Progress the fade effect.
    let step = config.fade_speed * time.delta_seconds();

    for (entity, mut fade) in query.iter_mut() {
        match *fade {
            Fade::In(progress) => {
                if progress < MAX_FADE_PROGRESS {
                    *fade = Fade::In(progress.max(MIN_FADE_PROGRESS) + step);
                } else {
                    commands.entity(entity).remove::<Fade>();
                }
            },
            Fade::Out(progress) => {
                if progress < MAX_FADE_PROGRESS {
                    *fade = Fade::Out(progress.max(MIN_FADE_PROGRESS) + step);
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
            FadeAnimation::Scaling {
                max_scale,
                axis_mask,
            } => {
                transform.scale =
                    (max_scale * axis_mask) * weight + (Vec3::ONE - axis_mask);
            },
            FadeAnimation::Translucency => {
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
