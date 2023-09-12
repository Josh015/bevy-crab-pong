use bevy::prelude::{App, Plugin};

pub mod arena;
pub mod ball;
pub mod debug_mode;
pub mod goal;
pub mod hud;
pub mod menu;
pub mod movement;
pub mod object;
pub mod ocean;
pub mod paddle;
pub mod resources;
pub mod side;
pub mod spawning;
pub mod state;
pub mod swaying_camera;
pub mod team;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            arena::ArenaPlugin,
            ball::BallPlugin,
            debug_mode::DebugModePlugin,
            goal::GoalPlugin,
            hud::HudPlugin,
            menu::MenuPlugin,
            movement::MovementPlugin,
            object::ObjectPlugin,
            ocean::OceanPlugin,
            paddle::PaddlePlugin,
            resources::ResourcesPlugin,
            spawning::SpawningPlugin,
            state::StatePlugin,
            swaying_camera::SwayingCameraPlugin,
            team::TeamPlugin,
        ));
    }
}
