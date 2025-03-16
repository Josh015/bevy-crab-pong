mod ball;
mod camera;
mod collider;
mod crab;
mod fade;
mod for_states;
mod goal;
mod hit_points_ui;
mod motion;
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
pub use hit_points_ui::*;
pub use motion::*;
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
            HitPointsUiPlugin,
            MotionPlugin,
            PolePlugin,
            ScrollingTexturePlugin,
        ));
    }
}
