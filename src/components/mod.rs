pub mod ball;
pub mod collider;
pub mod crab;
pub mod fade;
pub mod goal;
pub mod movement;
pub mod pole;
pub mod scrolling_texture;
pub mod side;
pub mod swaying_camera;

use bevy::prelude::*;

pub(super) struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            collider::ColliderPlugin,
            crab::CrabPlugin,
            fade::FadePlugin,
            movement::MovementPlugin,
            pole::PolePlugin,
            scrolling_texture::ScrollingTexturePlugin,
            side::SidePlugin,
            swaying_camera::SwayingCameraPlugin,
        ));
    }
}
