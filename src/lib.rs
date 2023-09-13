use bevy::prelude::{App, Plugin};

pub mod arena;
pub mod assets;
pub mod ball;
pub mod barrier;
pub mod collider;
pub mod config;
pub mod debug_mode;
pub mod fade;
pub mod goal;
pub mod hud;
pub mod menu;
pub mod movement;
pub mod object;
pub mod ocean;
pub mod paddle;
pub mod side;
pub mod state;
pub mod swaying_camera;
pub mod team;
pub mod wall;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            arena::ArenaPlugin,
            assets::AssetsPlugin,
            collider::ColliderPlugin,
            config::ConfigPlugin,
            debug_mode::DebugModePlugin,
            fade::FadePlugin,
            goal::GoalPlugin,
            hud::HudPlugin,
            menu::MenuPlugin,
            movement::MovementPlugin,
            object::ObjectPlugin,
            ocean::OceanPlugin,
            paddle::PaddlePlugin,
            state::StatePlugin,
            swaying_camera::SwayingCameraPlugin,
        ))
        .add_plugins((team::TeamPlugin,));
    }
}
