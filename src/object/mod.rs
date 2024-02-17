pub mod ball;
pub mod crab;
pub mod pole;

use bevy::prelude::*;
use spew::prelude::*;

use crate::level::side::Side;

/// Objects that can be spawned via Spew.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Object {
    Ball,
    Crab,
    Pole,
}

pub(super) struct ObjectPlugin;

impl Plugin for ObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ball::BallPlugin, crab::CrabPlugin, pole::PolePlugin))
            .add_plugins((
                SpewPlugin::<Object, Vec3>::default(),
                SpewPlugin::<Object, Side>::default(),
            ));
    }
}
