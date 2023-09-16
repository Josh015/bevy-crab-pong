pub mod ball;
pub mod crab;
pub mod wall;

use bevy::prelude::*;
use spew::prelude::*;

use crate::level::side::Side;

/// Objects that can be spawned via Spew.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Object {
    Ball,
    Crab,
    Wall,
}

pub struct ObjectPlugin;

impl Plugin for ObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SpewPlugin::<Object, Vec3>::default(),
            SpewPlugin::<Object, Side>::default(),
        ))
        .add_plugins((
            ball::BallPlugin,
            crab::CrabPlugin,
            wall::WallPlugin,
        ));
    }
}
