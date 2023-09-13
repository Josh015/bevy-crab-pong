use bevy::prelude::{App, Plugin};

pub mod assets;
pub mod barrier;
pub mod beach;
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
pub mod side;
pub mod state;
pub mod swaying_camera;
pub mod team;
pub mod util;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            assets::AssetsPlugin,
            barrier::BarrierPlugin,
            beach::BeachPlugin,
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
            state::StatePlugin,
            swaying_camera::SwayingCameraPlugin,
        ))
        .add_plugins(team::TeamPlugin);
    }
}
