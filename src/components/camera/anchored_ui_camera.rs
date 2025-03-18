use bevy::prelude::*;
use bevy_ui_anchor::AnchorUiPlugin;

pub(super) struct AnchoredUiCameraPlugin;

impl Plugin for AnchoredUiCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AnchorUiPlugin::<AnchoredUiCamera>::new());
    }
}

/// The [`Camera3d`] entity to track for anchored UI elements.
#[derive(Component, Debug)]
#[require(Camera3d)]
pub struct AnchoredUiCamera;
