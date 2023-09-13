use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    utils::HashMap,
};
use serde::Deserialize;

use crate::level::side::Side;

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
    pub max_ball_count: u8,
    pub crabs: HashMap<Side, CrabConfig>,
}

#[derive(Debug, Deserialize)]
pub struct CrabConfig {
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

/// The currently-selected game mode.
#[derive(Debug, Default, Resource)]
pub struct GameMode(pub usize);

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameMode>();
    }
}
