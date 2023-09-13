use bevy::prelude::*;

use crate::{movement::Active, state::AppState};

pub const SPAWNING_DURATION_IN_SECONDS: f32 = 1.0;

/// Marks an entity to fade in/out and delay activation.
#[derive(Clone, Component, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[component(storage = "SparseSet")]
pub enum Fade {
    #[default]
    In,
    Out,
}

/// Contains the [`FadeAnimation`] progress for this entity.
#[derive(Clone, Component, Debug)]
pub struct FadeProgress(pub Timer);

impl Default for FadeProgress {
    fn default() -> Self {
        Self(Timer::from_seconds(
            SPAWNING_DURATION_IN_SECONDS,
            TimerMode::Once,
        ))
    }
}

/// Specifies an entity's spawning effect animation.
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

/// Assigns an entity an animation and gets it to start fading in.
#[derive(Bundle, Clone, Debug, Default)]
pub struct FadeAnimationBundle {
    pub fade_animation: FadeAnimation,
    pub fade_bundle: FadeBundle,
}

/// Provides basics to start fading an entity.
#[derive(Bundle, Clone, Debug, Default)]
pub struct FadeBundle {
    pub spawning_progress: FadeProgress,
    pub fade: Fade,
}

impl FadeBundle {
    pub fn fade_in() -> Self {
        Self {
            fade: Fade::In,
            ..default()
        }
    }

    pub fn fade_out() -> Self {
        Self {
            fade: Fade::Out,
            ..default()
        }
    }
}

pub struct FadePlugin;

impl Plugin for FadePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            animate_fade_effect_on_entity
                .chain()
                .run_if(not(in_state(AppState::Paused))),
        )
        .add_systems(Last, finish_fading);
    }
}

fn animate_fade_effect_on_entity(
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(
        &mut Transform,
        &Handle<StandardMaterial>,
        &Fade,
        &FadeAnimation,
        &mut FadeProgress,
    )>,
) {
    for (mut transform, material, fade, animation, mut progress) in &mut query {
        progress.0.tick(time.delta());

        let weight = if *fade == Fade::In {
            progress.0.percent()
        } else {
            1.0 - progress.0.percent()
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

fn finish_fading(
    mut commands: Commands,
    query: Query<(Entity, &Fade, &FadeProgress)>,
) {
    for (entity, fade, progress) in &query {
        if progress.0.finished() {
            match fade {
                Fade::In => {
                    commands
                        .entity(entity)
                        .insert(Active)
                        .remove::<FadeBundle>();
                    info!("Entity({:?}): Active", entity);
                },
                Fade::Out => {
                    commands.entity(entity).despawn_recursive();
                    info!("Entity({:?}): Despawned", entity);
                },
            }
        }
    }
}
