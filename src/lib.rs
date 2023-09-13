use bevy::prelude::{App, Plugin};

pub mod assets;
pub mod collider;
pub mod config;
pub mod debug_mode;
pub mod fade;
pub mod hud;
pub mod level;
pub mod menu;
pub mod movement;
pub mod object;
pub mod state;
pub mod team;
pub mod util;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            assets::AssetsPlugin,
            collider::ColliderPlugin,
            config::ConfigPlugin,
            debug_mode::DebugModePlugin,
            fade::FadePlugin,
            hud::HudPlugin,
            level::LevelPlugin,
            menu::MenuPlugin,
            movement::MovementPlugin,
            object::ObjectPlugin,
            state::StatePlugin,
            team::TeamPlugin,
        ));
    }
}
