pub mod assets;
pub mod competitors;
pub mod state;

use bevy::prelude::*;

use crate::common::collider::ColliderSet;

/// Set containing game rules systems.
#[derive(SystemSet, Clone, Hash, Debug, PartialEq, Eq)]
pub struct GameSet;

pub(super) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(PostUpdate, GameSet.after(ColliderSet))
            .add_plugins((
                assets::AssetsPlugin,
                competitors::CompetitorsPlugin,
                state::StatePlugin,
            ));
    }
}
