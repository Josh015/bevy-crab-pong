mod insert_after_fade_in;
mod remove_before_fade_out;

use std::{f32::EPSILON, time::Duration};

pub use insert_after_fade_in::*;
pub use remove_before_fade_out::*;

use bevy::prelude::*;

use crate::system_sets::StopWhenPausedSet;

use super::{Collider, Motion};

pub(super) struct FadePlugin;

impl Plugin for FadePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((InsertAfterFadeInPlugin, RemoveBeforeFadeOutPlugin))
            .add_observer(start_fading)
            .add_systems(
                PostUpdate,
                fade_transition_over_time.in_set(StopWhenPausedSet),
            )
            .add_systems(
                Last,
                clean_up_components_or_entities_after_they_finish_fading,
            );
    }
}

/// An event fired to make an entity start fading.
#[derive(Debug, Event)]
pub struct StartFading(pub Fade, pub Entity);

/// Which direction the entity
#[derive(Clone, Component, Copy, Debug, Default, Eq, PartialEq)]
pub enum Fade {
    #[default]
    In,
    Out,
}

/// Specifies how long an entity takes to fade.
#[derive(Clone, Component, Copy, Debug, Default, PartialEq)]
pub struct FadeDuration(pub Duration);

/// Specifies an entity's fade effect animation.
#[derive(Clone, Component, Copy, Debug, Default, PartialEq)]
pub enum FadeEffect {
    /// Uses alpha-blending to fade in/out an entity.
    ///
    /// Will take control of the entity's [`StandardMaterial`] by setting it to
    /// [`AlphaMode::Blend`] and adjusting its `base_color` alpha.
    #[default]
    Opacity,

    /// Uses scale to grow/shrink an entity with axis masked using 0/1.
    ///
    /// Will take control of the entity's [`Transform`] `scale`.
    ScaleAxisMask(Vec3),
}

#[derive(Clone, Component, Debug, Default, PartialEq)]
struct FadeTimer(Timer);

#[derive(Clone, Component, Copy, Debug, PartialEq)]
enum FadeTransition {
    Opacity(f32, f32, AlphaMode),
    Scale(Vec3, Vec3),
}

fn start_fading(
    trigger: Trigger<StartFading>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(
        &FadeEffect,
        &FadeDuration,
        &mut Transform,
        &MeshMaterial3d<StandardMaterial>,
    )>,
) {
    let StartFading(fade, entity) = trigger.event();
    let Ok((fade_effect, fade_duration, mut transform, material)) =
        query.get_mut(*entity)
    else {
        return;
    };

    match fade_effect {
        FadeEffect::Opacity => {
            let material = materials.get_mut(material).unwrap();
            let (start, end, alpha_mode) = match *fade {
                Fade::In => {
                    (0.0, material.base_color.alpha(), material.alpha_mode)
                },
                Fade::Out => {
                    (material.base_color.alpha(), 0.0, AlphaMode::Blend)
                },
            };

            commands
                .entity(*entity)
                .insert(FadeTransition::Opacity(start, end, alpha_mode));
            material.alpha_mode = AlphaMode::Blend;
            material.base_color.set_alpha(start);
        },
        FadeEffect::ScaleAxisMask(axis_mask) => {
            let masked_start = transform.scale
                + (Vec3::splat(EPSILON) - transform.scale) * axis_mask;
            let (start, end) = match *fade {
                Fade::In => (masked_start, transform.scale),
                Fade::Out => (transform.scale, masked_start),
            };

            commands
                .entity(*entity)
                .insert(FadeTransition::Scale(start, end));
            transform.scale = start;
        },
    }

    commands.entity(*entity).insert((
        *fade,
        FadeTimer(Timer::new(fade_duration.0, TimerMode::Once)),
    ));
    info!("Entity({entity:?}): Start Fading");
}

fn fade_transition_over_time(
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(
        &mut FadeTimer,
        &FadeTransition,
        &mut Transform,
        &MeshMaterial3d<StandardMaterial>,
    )>,
) {
    for (mut fade_timer, fade_transition, mut transform, material) in &mut query
    {
        fade_timer.0.tick(time.delta());

        let weight = fade_timer.0.fraction();

        match *fade_transition {
            FadeTransition::Scale(start, end) => {
                transform.scale = start.lerp(end, weight);
            },
            FadeTransition::Opacity(start, end, alpha_mode) => {
                let material = materials.get_mut(material).unwrap();

                material.base_color.set_alpha(start.lerp(end, weight));

                if weight >= 1.0 {
                    material.alpha_mode = alpha_mode;
                }
            },
        }
    }
}

fn clean_up_components_or_entities_after_they_finish_fading(
    mut commands: Commands,
    query: Query<(Entity, &Fade, &FadeTimer)>,
) {
    for (entity, fade, fade_timer) in &query {
        if fade_timer.0.finished() {
            match fade {
                Fade::In => {
                    commands
                        .entity(entity)
                        .remove::<(Fade, FadeTimer, FadeTransition)>();
                    info!("Entity({entity:?}): Started Moving");
                },
                Fade::Out => {
                    commands.entity(entity).despawn_recursive();
                    info!("Entity({entity:?}): Despawned");
                },
            }
        }
    }
}
