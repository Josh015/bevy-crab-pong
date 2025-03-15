mod ball;
mod camera;
mod collider;
mod crab;
mod fade;
mod for_states;
mod goal;
mod movement;
mod pole;
mod scrolling_texture;
mod side;

pub use ball::*;
pub use camera::*;
pub use collider::*;
pub use crab::*;
pub use fade::*;
pub use for_states::*;
pub use goal::*;
pub use movement::*;
pub use pole::*;
pub use scrolling_texture::*;
pub use side::*;

use bevy::prelude::*;

pub(super) struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CameraPlugin,
            ColliderPlugin,
            CrabPlugin,
            FadePlugin,
            ForStatesPlugin,
            GoalPlugin,
            MovementPlugin,
            PolePlugin,
            ScrollingTexturePlugin,
        ));
    }
}
