#![allow(clippy::type_complexity)]

use crate::prelude::*;

/// Makes a [`FadeAnimation`] entity start its animation to fade out and
/// despawn.
fn fade_out_entity_event(
    mut commands: Commands,
    query: Query<(Entity, &Fade)>,
    mut event_reader: EventReader<FadeOutEntityEvent>,
) {
    for FadeOutEntityEvent(entity) in event_reader.iter() {
        // If interrupting Fade::In then start with its inverse progress to
        // avoid visual popping. If it's Fade::Out, just let it run until done.
        let progress = match query.get_component::<Fade>(*entity) {
            Ok(Fade::In(progress)) => 1.0 - *progress,
            Ok(Fade::Out(_)) => continue,
            _ => 0.0,
        };

        // Initiate fade out.
        commands.entity(*entity).insert(Fade::Out(progress));
        info!("Entity({:?}): Fading Out", entity);
    }
}

/// Progresses a [`Fade`] component to completion before either removing it or
/// despawning the entity.
fn fade_entities(
    mut commands: Commands,
    game_config: Res<GameConfig>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Fade), With<FadeAnimation>>,
) {
    for (entity, mut fade) in &mut query {
        // Progress the fade effect.
        let step = game_config.fade_speed * time.delta_seconds();

        match *fade {
            Fade::In(ref mut progress) => {
                if *progress < FADE_PROGRESS_MAX {
                    *progress = progress.max(FADE_PROGRESS_MIN) + step;
                } else {
                    info!("Entity({:?}): Ready", entity);
                    commands.entity(entity).remove::<Fade>();
                }
            },
            Fade::Out(ref mut progress) => {
                if *progress < FADE_PROGRESS_MAX {
                    *progress = progress.max(FADE_PROGRESS_MIN) + step;
                } else {
                    info!("Entity({:?}): Despawned", entity);
                    commands.entity(entity).despawn_recursive();
                }
            },
        }
    }
}

/// Handles [`Fade`] animations and the transition from visible->invisible and
/// vice versa over time.
fn fade_animation(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(
        &mut Transform,
        &Handle<StandardMaterial>,
        &FadeAnimation,
        &Fade,
    )>,
) {
    // Apply effect animation to the entity.
    for (mut transform, material, fade_animation, fade) in &mut query {
        let weight = match *fade {
            Fade::In(progress) => progress,
            Fade::Out(progress) => 1.0 - progress,
        };

        match *fade_animation {
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

pub struct DespawningPlugin;

impl Plugin for DespawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (fade_out_entity_event, fade_entities, fade_animation)
                .chain()
                .in_set(GameSystemSet::Despawning),
        );
    }
}
