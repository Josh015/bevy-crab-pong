use crate::prelude::*;

mod animated_water;
mod swaying_camera;
mod ui;

pub use animated_water::*;
pub use swaying_camera::*;
pub use ui::*;

pub const ARENA_CENTER_POINT: Vec3 = Vec3::ZERO;

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AnimatedWaterPlugin, SwayingCameraPlugin, UiPlugin));
    }
}
