use bevy::prelude::*;
use spew::prelude::*;

/// Objects that can be spawned via Spew.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Object {
    Ball,
    Wall,
    Crab,
    Barrier,
}

pub struct ObjectPlugin;

impl Plugin for ObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SpewPlugin::<Object>::default())
            .add_plugins(SpewPlugin::<Object, Entity>::default());
    }
}
