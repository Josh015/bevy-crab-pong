mod insert_after_fade_in;
mod remove_before_fade_out;

pub use insert_after_fade_in::*;
pub use remove_before_fade_out::*;

use bevy::prelude::*;
use derive_new::new;

use crate::system_sets::StopWhenPausedSet;

use super::{Collider, Motion};

pub(super) struct FadePlugin;

impl Plugin for FadePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((InsertAfterFadeInPlugin, RemoveBeforeFadeOutPlugin))
            .add_systems(
                PostUpdate,
                animate_fade_effect.in_set(StopWhenPausedSet),
            )
            .add_systems(
                Last,
                clean_up_components_or_entities_after_they_finish_fading,
            );
    }
}

/// Makes an entity fade in/out and delay activation/despawning respectively.
#[derive(Clone, Component, Debug, Eq, new, PartialEq)]
#[require(FadeEffect)]
#[component(storage = "SparseSet")]
pub enum Fade {
    In(#[new(value = "Timer::from_seconds(1.0, TimerMode::Once)")] Timer),
    Out(#[new(value = "Timer::from_seconds(1.0, TimerMode::Once)")] Timer),
}

/// Specifies an entity's fade effect animation.
#[derive(Clone, Component, Copy, Debug, Default, PartialEq)]
pub enum FadeEffect {
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

fn animate_fade_effect(
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(
        &mut Fade,
        &mut Transform,
        &MeshMaterial3d<StandardMaterial>,
        &FadeEffect,
    )>,
) {
    for (mut fade, mut transform, material, effect) in &mut query {
        let weight = match *fade {
            Fade::In(ref mut timer) => {
                timer.tick(time.delta());
                timer.fraction()
            },
            Fade::Out(ref mut timer) => {
                timer.tick(time.delta());
                1.0 - timer.fraction()
            },
        };

        match *effect {
            FadeEffect::Scale {
                max_scale,
                axis_mask,
            } => {
                transform.scale = max_scale
                    * (Vec3::ONE
                        + (Vec3::splat(weight) - Vec3::ONE) * axis_mask);
            },
            FadeEffect::Opacity => {
                let material = materials.get_mut(material).unwrap();

                material.base_color = material.base_color.with_alpha(weight);

                material.alpha_mode = if weight < 1.0 {
                    AlphaMode::Blend
                } else {
                    AlphaMode::Opaque
                };
            },
        }
    }
}

fn clean_up_components_or_entities_after_they_finish_fading(
    mut commands: Commands,
    query: Query<(Entity, &Fade)>,
) {
    for (entity, fade) in &query {
        match fade {
            Fade::In(timer) => {
                if timer.finished() {
                    commands.entity(entity).remove::<Fade>();
                    info!("Entity({entity:?}): Started Moving");
                }
            },
            Fade::Out(timer) => {
                if timer.finished() {
                    commands.entity(entity).despawn_recursive();
                    info!("Entity({entity:?}): Despawned");
                }
            },
        }
    }
}
