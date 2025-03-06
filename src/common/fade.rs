use std::marker::PhantomData;

use bevy::prelude::*;

use crate::game::state::PausableSet;

use super::{collider::Collider, movement::Movement};

pub const FADE_DURATION_IN_SECONDS: f32 = 1.0;

/// Makes an entity fade in/out and delay activation/despawning respectively.
#[derive(Clone, Component, Debug, Eq, PartialEq)]
#[require(FadeAnimation)]
#[component(storage = "SparseSet")]
pub enum Fade {
    In(Timer),
    Out(Timer),
}

impl Fade {
    pub fn new_in() -> Self {
        Self::In(Timer::from_seconds(
            FADE_DURATION_IN_SECONDS,
            TimerMode::Once,
        ))
    }

    pub fn new_out() -> Self {
        Self::Out(Timer::from_seconds(
            FADE_DURATION_IN_SECONDS,
            TimerMode::Once,
        ))
    }
}

impl Default for Fade {
    fn default() -> Self {
        Self::new_in()
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

/// Inserts a component after a fade-in finishes.
#[derive(Clone, Component, Copy, Debug, Default, PartialEq)]
pub struct InsertAfterFadeIn<B: Bundle + Default>(PhantomData<B>);

// Removes a component before a fade-out starts.
#[derive(Clone, Component, Copy, Debug, Default, PartialEq)]
pub struct RemoveBeforeFadeOut<B: Bundle>(PhantomData<B>);

pub(super) struct FadePlugin;

impl Plugin for FadePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, animate_fade_effect.in_set(PausableSet))
            .add_systems(
                Last,
                clean_up_components_or_entities_after_they_finish_fading,
            );
        app.add_systems(
            Update,
            (
                insert_component_after_fading_in::<Movement>,
                remove_component_before_fading_out::<Movement>,
                insert_component_after_fading_in::<Collider>,
                remove_component_before_fading_out::<Collider>,
            )
                .in_set(PausableSet),
        );
    }
}

fn animate_fade_effect(
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(
        &mut Fade,
        &mut Transform,
        &MeshMaterial3d<StandardMaterial>,
        &FadeAnimation,
    )>,
) {
    for (mut fade, mut transform, material, animation) in &mut query {
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

fn insert_component_after_fading_in<B: Bundle + Default>(
    mut commands: Commands,
    mut removed: RemovedComponents<Fade>,
    query: Query<Entity, With<InsertAfterFadeIn<B>>>,
) {
    // No need to exclude Fade::Out since the entity is already despawned.
    for entity in removed.read() {
        if query.contains(entity) {
            commands.entity(entity).insert(B::default());
        }
    }
}

fn remove_component_before_fading_out<B: Bundle>(
    mut commands: Commands,
    query: Query<(Entity, &Fade), (With<RemoveBeforeFadeOut<B>>, Added<Fade>)>,
) {
    for (entity, fade) in &query {
        if matches!(fade, Fade::Out(_)) {
            commands.entity(entity).remove::<B>();
        }
    }
}
