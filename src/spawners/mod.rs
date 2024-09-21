pub mod ball;
pub mod crab;
pub mod pole;

use bevy::prelude::*;

pub(super) struct SpawnersPlugin;

impl Plugin for SpawnersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ball::BallPlugin, crab::CrabPlugin, pole::PolePlugin));
    }
}
