use bevy::{
    ecs::system::SystemParam,
    prelude::*,
    reflect::{TypePath, TypeUuid},
    utils::HashMap,
};
use bevy_asset_loader::prelude::*;
use bevy_common_assets::yaml::YamlAssetPlugin;
use serde::Deserialize;
use std::{
    num::{NonZeroU8, NonZeroUsize},
    ops::Add,
};

use crate::level::side::Side;

use super::state::GameState;

/// Game settings read from a config file.
#[derive(Asset, Debug, Deserialize, Resource, TypePath, TypeUuid)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c46"]
pub struct GameConfig {
    pub pause_message: String,
    pub new_game_message: String,
    pub swaying_camera_speed: f32,
    pub ocean_scroll_speed: f32,
    pub team_win_messages: Vec<String>,
}

#[derive(Asset, Debug, Deserialize, Resource, TypePath, TypeUuid)]
#[uuid = "c6f093d2-c9b4-4334-a7d1-1a71876335cf"]
pub struct GameMode {
    pub name: String,
    pub ball_count: NonZeroU8,
    pub ball_size: f32,
    pub ball_speed: f32,
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
                meshes.add(
                    shape::UVSphere {
                        radius: 0.5,
                        sectors: 30,
                        stacks: 30,
                    }
                    .into(),
                ),
                // TODO: Replace with crab model.
                meshes.add(
                    shape::Capsule {
                        depth: 0.5,
                        latitudes: 10,
                        longitudes: 30,
                        radius: 0.5,
                        rings: 10,
                        ..default()
                    }
                    .into(),
                ),
                meshes.add(
                    shape::Cylinder {
                        height: 1.0,
                        radius: 0.5,
                        resolution: 20,
                        segments: 10,
                    }
                    .into(),
                ),
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

/// Allows systems to query and set the current game mode.
#[derive(SystemParam)]
pub struct GameModes<'w> {
    game_assets: Res<'w, GameAssets>,
    game_modes: Res<'w, Assets<GameMode>>,
    selected: ResMut<'w, SelectedGameMode>,
}

impl<'w> GameModes<'w> {
    /// Gets the current game mode.
    pub fn current(&self) -> &GameMode {
        self.game_modes
            .get(&self.game_assets.game_modes[self.selected.0])
            .unwrap()
    }

    /// Switch to the previous game mode.
    pub fn previous(&mut self) {
        self.selected.0 = self.selected.0.saturating_sub(1);
    }

    /// Switch to the next game mode.
    pub fn next(&mut self) {
        self.selected.0 = self
            .selected
            .0
            .add(1)
            .min(self.game_assets.game_modes.len() - 1);
    }
}

#[derive(Debug, Default, Resource)]
struct SelectedGameMode(usize);

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
                    .init_resource::<CachedAssets>()
                    .init_resource::<SelectedGameMode>(),
            );
    }
}
