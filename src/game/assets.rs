use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    utils::HashMap,
};
use bevy_asset_loader::prelude::*;
use serde::Deserialize;
use std::num::{NonZeroU8, NonZeroUsize};

use crate::level::side::Side;

/// Game settings read from a config file.
#[derive(Asset, Debug, Deserialize, Resource, TypeUuid, TypePath)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c46"]
pub struct GameConfig {
    pub pause_message: String,
    pub new_game_message: String,
    pub swaying_camera_speed: f32,
    pub ocean_scroll_speed: f32,
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
    pub team: NonZeroUsize,
    pub player: Player,
    pub hit_points: NonZeroU8,
    pub max_speed: f32,
    pub seconds_to_max_speed: f32,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
pub enum Player {
    Input,
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

    #[asset(key = "images.sand")]
    pub image_sand: Handle<Image>,

    #[asset(key = "images.water")]
    pub image_water: Handle<Image>,
}

/// Assets that need to remain loaded at all times.
#[derive(Debug, Resource)]
pub struct CachedAssets {
    pub ball_mesh: Handle<Mesh>,
    pub crab_mesh: Handle<Mesh>,
    pub pole_mesh: Handle<Mesh>,
    pub pole_material: Handle<StandardMaterial>,
}

impl FromWorld for CachedAssets {
    fn from_world(world: &mut World) -> Self {
        let (ball_mesh, crab_mesh, pole_mesh) = {
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
        let pole_material = {
            let mut materials = world
                .get_resource_mut::<Assets<StandardMaterial>>()
                .unwrap();

            materials.add(Color::hex("00A400").unwrap().into())
        };

        Self {
            ball_mesh,
            crab_mesh,
            pole_mesh,
            pole_material,
        }
    }
}

pub(super) struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CachedAssets>();
    }
}
