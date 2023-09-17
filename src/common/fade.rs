use bevy::prelude::*;

use crate::game::state::GameState;

pub const FADE_DURATION_IN_SECONDS: f32 = 1.0;

/// Makes an entity fade in/out and delay activation/despawning respectively.
#[derive(Clone, Component, Debug, Eq, PartialEq)]
#[component(storage = "SparseSet")]
pub enum Fade {
    In(Timer),
    Out(Timer),
}

impl Fade {
    pub fn out_default() -> Self {
        Self::Out(Timer::from_seconds(
            FADE_DURATION_IN_SECONDS,
            TimerMode::Once,
        ))
    }
}

impl Default for Fade {
    fn default() -> Self {
        Self::In(Timer::from_seconds(
            FADE_DURATION_IN_SECONDS,
            TimerMode::Once,
        ))
    }
}

/// Specifies an entity's fade effect animation.
#[derive(Clone, Component, Copy, Debug, Default, PartialEq)]
pub enum FadeAnimation {
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

/// Assigns an entity an animation and gets it to start fading.
#[derive(Bundle, Clone, Debug, Default)]
pub struct FadeBundle {
    pub fade_animation: FadeAnimation,
    pub fade: Fade,
}

pub(super) struct FadePlugin;

impl Plugin for FadePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            animate_fade_effect.run_if(not(in_state(GameState::Paused))),
        )
        .add_systems(
            Last,
            finish_fading.run_if(not(in_state(GameState::Paused))),
        );
    }
}

fn animate_fade_effect(
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(
        &mut Fade,
        &mut Transform,
        &Handle<StandardMaterial>,
        &FadeAnimation,
    )>,
) {
    for (mut fade, mut transform, material, animation) in &mut query {
        let weight = match *fade {
            Fade::In(ref mut progress) => {
                progress.tick(time.delta());
                progress.percent()
            },
            Fade::Out(ref mut progress) => {
                progress.tick(time.delta());
                1.0 - progress.percent()
            },
        };

        match *animation {
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

fn finish_fading(mut commands: Commands, query: Query<(Entity, &Fade)>) {
    for (entity, fade) in &query {
        match fade {
            Fade::In(progress) => {
                if progress.finished() {
                    commands.entity(entity).remove::<Fade>();
                    info!("Entity({:?}): Started Moving", entity);
                }
            },
            Fade::Out(progress) => {
                if progress.finished() {
                    commands.entity(entity).despawn_recursive();
                    info!("Entity({:?}): Despawned", entity);
                }
            },
        }
    }
}
