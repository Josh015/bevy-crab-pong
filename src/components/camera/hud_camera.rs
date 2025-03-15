use bevy::prelude::*;
use bevy_ui_anchor::AnchorUiPlugin;

pub(super) struct HudCameraPlugin;

impl Plugin for HudCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AnchorUiPlugin::<HudCamera>::new());
    }
}

#[derive(Component, Debug)]
#[require(Camera3d)]
pub struct HudCamera;
