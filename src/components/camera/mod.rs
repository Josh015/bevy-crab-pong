mod anchored_ui_camera;
mod swaying_camera;

pub use anchored_ui_camera::*;
pub use swaying_camera::*;

use bevy::prelude::*;

pub(super) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AnchoredUiCameraPlugin, SwayingCameraPlugin));
    }
}
