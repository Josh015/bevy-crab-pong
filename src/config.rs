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
    pub max_ball_count: usize,
    pub ball_max_speed: f32,
    pub ball_seconds_to_max_speed: f32,
    pub fade_speed: f32,
    pub starting_hit_points: u32,
    pub modes: Vec<ModeConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ModeConfig {
    pub name: String,
    pub goals: [GoalConfig; 4],
}

#[derive(Debug, Deserialize)]
pub struct GoalConfig {
    pub team: TeamConfig,
    pub controller: ControllerConfig,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub enum TeamConfig {
    Enemies,
    Allies,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub enum ControllerConfig {
    Keyboard,
    AI,
}
