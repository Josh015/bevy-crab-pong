use crate::prelude::*;

mod ai;
mod animated_water;
mod ball;
mod barrier;
mod collider;
mod fade;
mod for_state;
mod goal;
mod movement;
mod paddle;
mod player;
mod side;
mod swaying_camera;
mod ui;
mod wall;

pub use ai::*;
pub use animated_water::*;
pub use ball::*;
pub use barrier::*;
pub use collider::*;
pub use fade::*;
pub use for_state::*;
pub use goal::*;
pub use movement::*;
pub use paddle::*;
pub use player::*;
pub use side::*;
pub use swaying_camera::*;
pub use ui::*;
pub use wall::*;

pub const ARENA_CENTER_POINT: Vec3 = Vec3::ZERO;

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AnimatedWaterPlugin)
            .add_plugin(BallPlugin)
            .add_plugin(ColliderPlugin)
            .add_plugin(AiPlugin)
            .add_plugin(FadePlugin)
            .add_plugin(ForStatePlugin)
            .add_plugin(GoalPlugin)
            .add_plugin(MovementPlugin)
            .add_plugin(PaddlePlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(SwayingCameraPlugin)
            .add_plugin(UiPlugin)
            .add_plugin(WallPlugin);
    }
}
