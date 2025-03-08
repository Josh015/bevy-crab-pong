pub mod assets;
pub mod competitors;
pub mod events;
pub mod level;
pub mod modes;
pub mod state;

use bevy::prelude::*;

pub(super) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            assets::AssetsPlugin,
            competitors::CompetitorsPlugin,
            events::EventsPlugin,
            level::LevelPlugin,
            modes::ModesPlugin,
            state::StatePlugin,
        ));
    }
}
