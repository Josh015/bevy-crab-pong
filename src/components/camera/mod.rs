mod swaying_camera;
mod ui_camera;

pub use swaying_camera::*;
pub use ui_camera::*;

use bevy::prelude::*;

pub(super) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SwayingCameraPlugin, UiCameraPlugin));
    }
}
