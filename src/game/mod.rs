pub mod assets;
pub mod competitors;
pub mod state;

use bevy::prelude::*;

pub(super) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            assets::AssetsPlugin,
            competitors::CompetitorsPlugin,
            state::StatePlugin,
        ));
    }
}
