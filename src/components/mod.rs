pub mod ball;
pub mod camera;
pub mod collider;
pub mod crab;
pub mod fade;
pub mod goal;
pub mod movement;
pub mod pole;
pub mod scrolling_texture;
pub mod side;

use bevy::prelude::*;

pub(super) struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            camera::CameraPlugin,
            collider::ColliderPlugin,
            crab::CrabPlugin,
            fade::FadePlugin,
            goal::GoalPlugin,
            movement::MovementPlugin,
            pole::PolePlugin,
            scrolling_texture::ScrollingTexturePlugin,
        ));
    }
}
