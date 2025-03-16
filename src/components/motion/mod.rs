mod acceleration;
mod direction;
mod speed;

pub use acceleration::*;
pub use direction::*;
pub use speed::*;

use bevy::prelude::*;

pub(super) struct MotionPlugin;

impl Plugin for MotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AccelerationPlugin, SpeedPlugin));
    }
}

/// Marks an entity as in-motion and moving.
#[derive(Component, Default)]
pub struct Motion;
