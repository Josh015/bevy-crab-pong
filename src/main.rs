mod entities;
mod file;
mod game;

pub mod prelude {
    pub use crate::{entities::*, file::*, game::*};
    pub use bevy::{math::*, prelude::*};
    pub use rand::prelude::*;
}

use crate::prelude::*;
use bevy::window::{PresentMode, WindowResolution};

fn main() {
    let game_config: GameConfig =
        load_config_from_file("assets/config/game.ron");

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: game_config.title.clone(),
                resolution: WindowResolution::new(
                    game_config.width as f32,
                    game_config.height as f32,
                ),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Msaa::default())
        .insert_resource(ClearColor(game_config.clear_color))
        .insert_resource(game_config)
        .add_plugins((EntitiesPlugin, GamePlugin))
        .run();
}
