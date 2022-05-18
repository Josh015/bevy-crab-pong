use crate::prelude::*;

/// The type of fade effect animation to use.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FadeEffect {
    /// Effect that uses material opacity and alpha blending.
    Translucent,

    /// Effect that controls the transform scale of the entity.
    Scale { max_scale: Vec3, axis_mask: Vec3 },
}

/// Whether the effect is currently fading in or out.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FadeState {
    /// Simulates a fade-in effect using a weight in the range \[0,1\].
    In(f32),

    /// Simulates a fade-out effect using a weight in the range \[0,1\].
    Out(f32),
}

impl FadeState {
    /// Returns the opacity of the current state.
    pub fn opacity(&self) -> f32 {
        match *self {
            Self::In(weight) => weight,
            Self::Out(weight) => 1.0 - weight,
        }
    }
}

/// A component that handles fading an entity in/out of visibility and
/// despawning it if necessary.
#[derive(Clone, Component, Copy, PartialEq, Debug)]
pub struct Fade {
    /// The type of fade effect.
    pub effect: FadeEffect,

    /// The current state of the fade effect.
    pub state: Option<FadeState>,
}

impl Fade {
    /// Makes this entity fade out and then despawn itself.
    pub fn fade_out_and_despawn(&mut self) {
        // If interrupting a fade-in then start the fade-out with its inverse
        // weight to minimize visual popping. If already fading out it will
        // just restart it, but that's too much of an edge case to bother.
        self.state = Some(FadeState::Out(
            if let Some(FadeState::In(weight)) = self.state {
                1.0 - weight
            } else {
                0.0
            },
        ));
    }
}

impl Default for Fade {
    fn default() -> Self {
        Self {
            effect: FadeEffect::Translucent,
            state: Some(FadeState::In(0.0)),
        }
    }
}

/// Handles `Fade` effects' animations and the transition from
/// visible->invisible and vice versa over time.
pub fn fade_animation_system(
    mut commands: Commands,
    config: Res<GameConfig>,
    time: Res<Time>,
    state: Res<State<AppState>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(
        Entity,
        &mut Transform,
        &mut Fade,
        &Handle<StandardMaterial>,
    )>,
) {
    // Prevent fade animations from running when game is paused.
    if *state.current() == AppState::Pause {
        return;
    }

    // Animate the entity.
    let step = config.fade_speed * time.delta_seconds();

    for (entity, mut transform, mut fade, material) in query.iter_mut() {
        if let Some(state) = fade.state {
            // Apply effect animation.
            let opacity = state.opacity();

            match fade.effect {
                FadeEffect::Scale {
                    max_scale,
                    axis_mask,
                } => {
                    transform.scale = (max_scale * axis_mask) * opacity
                        + (Vec3::ONE - axis_mask);
                },
                FadeEffect::Translucent => {
                    let material = materials.get_mut(material).unwrap();

                    material.base_color.set_a(opacity);
                    material.alpha_mode = if opacity < 1.0 {
                        AlphaMode::Blend
                    } else {
                        AlphaMode::Opaque
                    };
                },
            }

            // Update progress of the effect.
            match state {
                FadeState::In(weight) => {
                    if weight < 1.0 {
                        fade.state =
                            Some(FadeState::In(weight.max(0.0) + step));
                    } else {
                        fade.state = None;
                    }
                },
                FadeState::Out(weight) => {
                    if weight < 1.0 {
                        fade.state =
                            Some(FadeState::Out(weight.max(0.0) + step));
                    } else {
                        fade.state = None;
                        commands.entity(entity).despawn_recursive();
                    }
                },
            }
        }
    }
}
