use bevy::prelude::*;
use bevy_ui_anchor::AnchorUiPlugin;

pub(super) struct UiCameraPlugin;

impl Plugin for UiCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AnchorUiPlugin::<UiCamera>::new());
    }
}

/// The [`Camera3d`] entity to track for anchored UI elements.
#[derive(Component, Debug)]
#[require(Camera3d)]
pub struct UiCamera;
