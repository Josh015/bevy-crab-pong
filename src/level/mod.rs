pub mod barrier;
pub mod beach;
pub mod goal;
pub mod ocean;
pub mod side;
pub mod swaying_camera;

use bevy::prelude::*;

pub(super) struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            barrier::BarrierPlugin,
            beach::BeachPlugin,
            goal::GoalPlugin,
            ocean::OceanPlugin,
            swaying_camera::SwayingCameraPlugin,
        ));
    }
}
