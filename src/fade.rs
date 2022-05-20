use crate::prelude::*;

/// A component that specifies the entity's fade effect animation.
#[derive(Clone, Component, Copy, PartialEq, Debug)]
pub enum FadeAnimation {
    /// Effect that uses material opacity and alpha blending.
    Translucency,

    /// Effect that controls the transform scale of the entity.
    Scaling { max_scale: Vec3, axis_mask: Vec3 },
}

/// A component that makes an entity fade in/out and then despawn if needed.
#[derive(Clone, Component, Copy, PartialEq, Debug)]
pub enum Fade {
    /// Simulates a fade-in effect using a weight in the range \[0,1\].
    In(f32),

    /// Simulates a fade-out effect using a weight in the range \[0,1\].
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
            Fade::In(weight) => {
                if weight < 1.0 {
                    *fade = Fade::In(weight.max(0.0) + step);
                } else {
                    commands.entity(entity).remove::<Fade>();
                }
            },
            Fade::Out(weight) => {
                if weight < 1.0 {
                    *fade = Fade::Out(weight.max(0.0) + step);
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
    // Animate the entity.
    for (mut transform, material, fade_effect, fade) in query.iter_mut() {
        // Apply effect animation.
        let opacity = match *fade {
            Fade::In(weight) => weight,
            Fade::Out(weight) => 1.0 - weight,
        };

        match *fade_effect {
            FadeAnimation::Scaling {
                max_scale,
                axis_mask,
            } => {
                transform.scale =
                    (max_scale * axis_mask) * opacity + (Vec3::ONE - axis_mask);
            },
            FadeAnimation::Translucency => {
                let material = materials.get_mut(material).unwrap();

                material.base_color.set_a(opacity);
                material.alpha_mode = if opacity < 1.0 {
                    AlphaMode::Blend
                } else {
                    AlphaMode::Opaque
                };
            },
        }
    }
}
