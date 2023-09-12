use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    text::Font,
    utils::HashMap,
};
use bevy_asset_loader::prelude::*;
use serde::Deserialize;

use crate::components::Side;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(key = "game.config")]
    pub game_config: Handle<GameConfig>,

    #[asset(key = "fonts.menu")]
    pub font_menu: Handle<Font>,

    #[asset(key = "images.paddle")]
    pub image_paddle: Handle<Image>,
}

/// Game settings read from a config file.
#[derive(Debug, Deserialize, Resource, TypeUuid, TypePath)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c46"]
pub struct GameConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub pause_message: String,
    pub new_game_message: String,
    pub clear_color: Color,
    pub swaying_camera_speed: f32,
    pub animated_water_speed: f32,
    pub paddle_max_speed: f32,
    pub paddle_seconds_to_max_speed: f32,
    pub ball_speed: f32,
    pub spawn_speed: f32,
    pub team_win_messages: Vec<String>,
    pub modes: Vec<ModeConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ModeConfig {
    pub name: String,
    pub max_ball_count: usize,
    pub paddles: HashMap<Side, PaddleConfig>,
}

#[derive(Debug, Deserialize)]
pub struct PaddleConfig {
    pub color: String,
    pub team: usize,
    pub player: PlayerConfig,
    pub hit_points: u8,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
pub enum PlayerConfig {
    Keyboard,
    AI,
}
