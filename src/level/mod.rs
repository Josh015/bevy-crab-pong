pub mod beach;
pub mod ocean;
pub mod side;
pub mod swaying_camera;

use bevy::prelude::*;

pub(super) struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            beach::BeachPlugin,
            ocean::OceanPlugin,
            side::SidePlugin,
            swaying_camera::SwayingCameraPlugin,
        ));
    }
}
