use crate::prelude::*;

/// The type of fade effect animation to use.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FadeEffect {
    Translucent,
    Scale { max_scale: Vec3, axis_mask: Vec3 },
}

/// Whether the effect is currently fading in or out.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FadeState {
    /// Simulates a fade-in effect, using a weight in the range \[0,1\].
    In(f32),

    /// Simulates a fade-out effect, using a weight in the range \[0,1\].
    Out(f32),
}

impl FadeState {
    /// Returns the opacity of the current `Fade` type.
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
    effect: FadeEffect,

    /// The current state of the fade of effect.
    state: Option<FadeState>,
}

impl Fade {
    pub fn new(effect: FadeEffect) -> Self {
        Self::new_with_starting_state(effect, Some(FadeState::In(0.0)))
    }

    pub fn new_with_starting_state(
        effect: FadeEffect,
        state: Option<FadeState>,
    ) -> Self {
        Self { effect, state }
    }

    pub fn fade_out_and_despawn(&mut self) {
        self.state = Some(FadeState::Out(0.0));
    }

    pub fn state(&self) -> Option<FadeState> {
        self.state
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
    // Need to pause animations so balls launch correctly.
    if *state.current() == AppState::Pause {
        return;
    }

    // Animate the entity.
    let step = config.fade_speed * time.delta_seconds();

    for (entity, mut transform, mut fade, material) in query.iter_mut() {
        match fade.state {
            Some(state) => {
                // Apply effect animation.
                match fade.effect {
                    FadeEffect::Scale {
                        max_scale,
                        axis_mask,
                    } => {
                        transform.scale = (max_scale * axis_mask)
                            * state.opacity()
                            + (Vec3::ONE - axis_mask);
                    },
                    FadeEffect::Translucent => {
                        let material = materials.get_mut(material).unwrap();
                        let opacity = state.opacity();

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
            },
            _ => {},
        }
    }
}
