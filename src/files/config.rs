use crate::prelude::*;
use serde::Deserialize;

/// Game settings read from a `*.ron` config file.
#[derive(Debug, Deserialize, Resource)]
pub struct GameConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub pause_message: String,
    pub game_over_win_message: String,
    pub game_over_lose_message: String,
    pub new_game_message: String,
    pub clear_color: Color,
    pub swaying_camera_speed: f32,
    pub animated_water_speed: f32,
    pub paddle_max_speed: f32,
    pub paddle_seconds_to_max_speed: f32,
    pub ball_speed: f32,
    pub fade_speed: f32,
    pub modes: Vec<ModeConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ModeConfig {
    pub name: String,
    pub max_ball_count: usize,
    pub goals: [GoalConfig; 4],
}

#[derive(Debug, Deserialize)]
pub struct GoalConfig {
    pub color: String,
    pub team: TeamConfig,
    pub controlled_by: ControlledByConfig,
    pub starting_hit_points: u32,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub enum TeamConfig {
    Enemies,
    Allies,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub enum ControlledByConfig {
    Keyboard,
    AI,
}
