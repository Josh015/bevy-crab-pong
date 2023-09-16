pub mod assets;
pub mod collider;
pub mod debug_mode;
pub mod fade;
pub mod game;
pub mod level;
pub mod movement;
pub mod object;
pub mod player;
pub mod state;
pub mod ui;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            assets::AssetsPlugin,
            collider::ColliderPlugin,
            debug_mode::DebugModePlugin,
            fade::FadePlugin,
            game::GamePlugin,
            level::LevelPlugin,
            movement::MovementPlugin,
            object::ObjectPlugin,
            player::PlayerPlugin,
            state::StatePlugin,
            ui::UiPlugin,
        ));
    }
}
