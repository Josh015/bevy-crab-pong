use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    utils::HashMap,
};
use bevy_asset_loader::prelude::*;
use serde::Deserialize;
use std::num::NonZeroU8;

use crate::side::Side;

/// Game settings read from a config file.
#[derive(Debug, Deserialize, Resource, TypeUuid, TypePath)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c46"]
pub struct GameConfig {
    pub pause_message: String,
    pub new_game_message: String,
    pub swaying_camera_speed: f32,
    pub ocean_scroll_speed: f32,
    pub crab_max_speed: f32,
    pub crab_seconds_to_max_speed: f32,
    pub ball_speed: f32,
    pub team_win_messages: Vec<String>,
    pub modes: Vec<ModeConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ModeConfig {
    pub name: String,
    pub ball_count: NonZeroU8,
    pub competitors: HashMap<Side, CompetitorConfig>,
}

#[derive(Debug, Deserialize)]
pub struct CompetitorConfig {
    pub color: String,
    pub team: usize,
    pub player: PlayerConfig,
    pub hit_points: NonZeroU8,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
pub enum PlayerConfig {
    Keyboard,
    AI,
}

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(key = "game.config")]
    pub game_config: Handle<GameConfig>,

    #[asset(key = "fonts.menu")]
    pub font_menu: Handle<Font>,

    #[asset(key = "images.crab")]
    pub image_crab: Handle<Image>,
}

/// Assets that need to remain loaded at all times.
#[derive(Debug, Resource)]
pub struct CachedAssets {
    pub ball_mesh: Handle<Mesh>,
    pub crab_mesh: Handle<Mesh>,
    pub wall_mesh: Handle<Mesh>,
    pub wall_material: Handle<StandardMaterial>,
}

impl FromWorld for CachedAssets {
    fn from_world(world: &mut World) -> Self {
        let (ball_mesh, crab_mesh, wall_mesh) = {
            let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();

            (
                meshes.add(Mesh::from(shape::UVSphere {
                    radius: 0.5,
                    sectors: 30,
                    stacks: 30,
                })),
                // TODO: Replace with crab model.
                meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            )
        };
        let wall_material = {
            let mut materials = world
                .get_resource_mut::<Assets<StandardMaterial>>()
                .unwrap();

            materials.add(Color::hex("00A400").unwrap().into())
        };

        Self {
            ball_mesh,
            crab_mesh,
            wall_mesh,
            wall_material,
        }
    }
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CachedAssets>();
    }
}
