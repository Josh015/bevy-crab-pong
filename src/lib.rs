use bevy::prelude::*;

pub mod assets;
pub mod ball;
pub mod barrier;
pub mod beach;
pub mod collider;
pub mod crab;
pub mod debug_mode;
pub mod fade;
pub mod game;
pub mod goal;
pub mod hud;
pub mod menu;
pub mod movement;
pub mod object;
pub mod ocean;
pub mod player;
pub mod side;
pub mod state;
pub mod swaying_camera;
pub mod wall;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            assets::AssetsPlugin,
            beach::BeachPlugin,
            collider::ColliderPlugin,
            crab::CrabPlugin,
            debug_mode::DebugModePlugin,
            fade::FadePlugin,
            game::GamePlugin,
            goal::GoalPlugin,
            hud::HudPlugin,
            menu::MenuPlugin,
            movement::MovementPlugin,
            object::ObjectPlugin,
            ocean::OceanPlugin,
            player::PlayerPlugin,
            state::StatePlugin,
        ))
        .add_plugins(swaying_camera::SwayingCameraPlugin);
    }
}
