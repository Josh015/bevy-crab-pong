pub mod hud_camera;
pub mod swaying_camera;

use bevy::prelude::*;

pub(super) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            hud_camera::HudCameraPlugin,
            swaying_camera::SwayingCameraPlugin,
        ));
    }
}
