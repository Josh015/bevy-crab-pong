use bevy::prelude::*;

use crate::{
    components::spawning::{
        Despawning, SpawnAnimation, SpawnProgress, Spawning,
    },
    constants::*,
    serialization::{GameAssets, GameConfig},
};

use super::GameSystemSet;

fn start_despawning_entity(
    mut commands: Commands,
    query: Query<(Entity, Option<&Spawning>), Added<Despawning>>,
) {
    for (entity, spawning) in &query {
        if spawning.is_some() {
            commands.entity(entity).remove::<Spawning>();
        }

        info!("Entity({:?}): Despawning", entity);
    }
}

fn advance_spawning_progress(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    game_configs: Res<Assets<GameConfig>>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut SpawnProgress), With<Spawning>>,
) {
    let game_config = game_configs.get(&game_assets.game_config).unwrap();

    for (entity, mut progress) in &mut query {
        let step = game_config.fade_speed * time.delta_seconds();

        if progress.0 < FADE_PROGRESS_MAX {
            progress.0 = progress.0.max(FADE_PROGRESS_MIN) + step;
        } else {
            commands.entity(entity).remove::<Spawning>();
            info!("Entity({:?}): Spawned", entity);
        }
    }
}

fn advance_despawning_progress(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    game_configs: Res<Assets<GameConfig>>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut SpawnProgress), With<Despawning>>,
) {
    let game_config = game_configs.get(&game_assets.game_config).unwrap();

    for (entity, mut progress) in &mut query {
        let step = game_config.fade_speed * time.delta_seconds();

        if progress.0 > FADE_PROGRESS_MIN {
            progress.0 = progress.0.min(FADE_PROGRESS_MAX) - step;
        } else {
            commands.entity(entity).remove::<Despawning>();
            info!("Entity({:?}): Despawned", entity);
        }
    }
}

fn animate_fade_effect_on_entity(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<
        (
            &mut Transform,
            &Handle<StandardMaterial>,
            &SpawnAnimation,
            &SpawnProgress,
        ),
        Or<(With<Spawning>, With<Despawning>)>,
    >,
) {
    for (mut transform, material, animation, progress) in &mut query {
        match *animation {
            SpawnAnimation::Scale {
                max_scale,
                axis_mask,
            } => {
                transform.scale = (max_scale * axis_mask) * progress.0
                    + (Vec3::ONE - axis_mask);
            },
            SpawnAnimation::Opacity => {
                let material = materials.get_mut(material).unwrap();

                material.base_color.set_a(progress.0);
                material.alpha_mode = if progress.0 < 1.0 {
                    AlphaMode::Blend
                } else {
                    AlphaMode::Opaque
                };
            },
        }
    }
}

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                start_despawning_entity,
                advance_spawning_progress,
                advance_despawning_progress,
                animate_fade_effect_on_entity,
            )
                .chain()
                .in_set(GameSystemSet::Effects),
        );
    }
}
