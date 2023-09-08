use bevy::prelude::{Color, Resource};
use ron::de::from_reader;
use serde::{de::DeserializeOwned, Deserialize};
use std::{fs::File, path::PathBuf};

/// Game settings read from a config file.
#[derive(Debug, Deserialize, Resource)]
pub struct Config {
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

/// Opens a file using the project's manifest directory as the root.
pub fn open_local_file(path: &str) -> File {
    let input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path);
    File::open(input_path)
        .expect(&format!("Failed opening file: {:#?}", path)[..])
}

/// Opens and loads a `*.ron` config file into a compatible struct.
pub fn load_config_from_ron_file<T: DeserializeOwned>(path: &str) -> T {
    let f = open_local_file(path);

    match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Failed to load config: {}", e);

            std::process::exit(1);
        },
    }
}
