use bevy::prelude::{App, Plugin};

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
pub mod side;
pub mod state;
pub mod swaying_camera;
pub mod util;
pub mod wall;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            assets::AssetsPlugin,
            ball::BallPlugin,
            barrier::BarrierPlugin,
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
        ))
        .add_plugins((
            state::StatePlugin,
            swaying_camera::SwayingCameraPlugin,
            wall::WallPlugin,
        ));
    }
}
