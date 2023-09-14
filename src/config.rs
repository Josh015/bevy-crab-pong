use std::num::NonZeroU8;

use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    utils::HashMap,
};
use serde::Deserialize;

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
    pub max_ball_count: NonZeroU8,
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
