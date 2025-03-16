use bevy::{prelude::*, reflect::TypePath, utils::HashMap};
use bevy_asset_loader::prelude::*;
use bevy_common_assets::yaml::YamlAssetPlugin;
use serde::Deserialize;
use std::num::{NonZeroU8, NonZeroUsize};

use crate::{components::Side, states::GameState};

pub(super) struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(YamlAssetPlugin::<GameConfig>::new(&["config.yaml"]))
            .add_plugins(YamlAssetPlugin::<GameMode>::new(&["mode.yaml"]))
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    .continue_to_state(GameState::StartMenu),
            )
            .configure_loading_state(
                LoadingStateConfig::new(GameState::Loading)
                    .load_collection::<GameAssets>()
                    .init_resource::<CachedAssets>(),
            );
    }
}

/// Game settings read from a config file.
#[derive(Asset, Debug, Deserialize, Resource, TypePath)]
pub struct GameConfig {
    pub swaying_camera_speed: f32,
    pub ocean_scroll_speed: f32,
    pub beach_width: f32,
    pub barrier_diameter: f32,
    pub barrier_height: f32,
    pub crab_width: f32,
    pub crab_depth: f32,
    pub crab_height_from_ground: f32,
    pub pole_diameter: f32,
    pub pole_height_from_ground: f32,
    pub ball_diameter: f32,
    pub ball_height_from_ground: f32,
    pub new_game_message: String,
    pub pause_message: String,
    pub team_win_messages: Vec<String>,
}

#[derive(Asset, Debug, Deserialize, Resource, TypePath)]
pub struct GameMode {
    pub name: String,
    pub ball_count: NonZeroU8,
    pub ball_scale: f32,
    pub ball_speed: f32,
    pub competitors: HashMap<Side, CompetitorConfig>,
}

#[derive(Debug, Deserialize)]
pub struct CompetitorConfig {
    pub color: String,
    pub team: NonZeroUsize,
    pub controller: CrabController,
    pub hit_points: NonZeroU8,
    pub max_speed: f32,
    pub seconds_to_max_speed: f32,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
pub enum CrabController {
    Player,
    AI,
}

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "game.config.yaml")]
    pub game_config: Handle<GameConfig>,

    #[asset(path = "modes", collection(typed))]
    pub game_modes: Vec<Handle<GameMode>>,

    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub font_menu: Handle<Font>,

    #[asset(path = "images/crab.png")]
    pub image_crab: Handle<Image>,

    #[asset(path = "images/sand.png")]
    pub image_sand: Handle<Image>,

    #[asset(path = "images/water.png")]
    #[asset(image(sampler(filter = linear, wrap = repeat)))]
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
                meshes.add(Sphere { radius: 0.5 }),
                // TODO: Replace with crab model.
                meshes.add(Capsule3d {
                    half_length: 0.25,
                    radius: 0.5,
                }),
                meshes.add(Cylinder {
                    half_height: 0.5,
                    radius: 0.5,
                }),
            )
        };
        let pole_material = {
            let mut materials = world
                .get_resource_mut::<Assets<StandardMaterial>>()
                .unwrap();

            materials.add(Color::Srgba(Srgba::hex("00A400").unwrap()))
        };

        Self {
            ball_mesh,
            crab_mesh,
            pole_mesh,
            pole_material,
        }
    }
}
