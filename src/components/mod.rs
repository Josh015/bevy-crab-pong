use crate::prelude::*;

mod ai;
mod animated_water;
mod ball;
mod barrier;
mod collider;
mod fade;
mod for_state;
mod gizmos;
mod goal;
mod movement;
mod paddle;
mod player;
mod side;
mod swaying_camera;
mod targeting;
mod ui;
mod wall;

pub use ai::*;
pub use animated_water::*;
pub use ball::*;
pub use barrier::*;
pub use collider::*;
pub use fade::*;
pub use for_state::*;
pub use gizmos::*;
pub use goal::*;
pub use movement::*;
pub use paddle::*;
pub use player::*;
pub use side::*;
pub use swaying_camera::*;
pub use targeting::*;
pub use ui::*;
pub use wall::*;

pub const ARENA_CENTER_POINT: Vec3 = Vec3::ZERO;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum LogicalSet {
    GameplayLogic,
    Movement,
    Collision,
    Despawning,
}

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            Update,
            LogicalSet::GameplayLogic
                .before(LogicalSet::Movement)
                .run_if(in_state(GameScreen::Playing)),
        );
        app.configure_set(
            Update,
            LogicalSet::Movement.run_if(not(in_state(GameScreen::Paused))),
        );
        app.configure_set(
            PostUpdate,
            LogicalSet::Collision
                .after(LogicalSet::Movement)
                .before(LogicalSet::Despawning)
                .run_if(in_state(GameScreen::Playing)),
        );
        app.configure_set(
            PostUpdate,
            LogicalSet::Despawning
                .after(LogicalSet::Movement)
                .run_if(not(in_state(GameScreen::Paused))),
        );
        app.add_plugins((
            AnimatedWaterPlugin,
            BallPlugin,
            ColliderPlugin,
            AiPlugin,
            FadePlugin,
            ForStatePlugin,
            GizmosPlugin,
            GoalPlugin,
            MovementPlugin,
            PaddlePlugin,
            PlayerPlugin,
            SwayingCameraPlugin,
            TargetingPlugin,
            UiPlugin,
            WallPlugin,
        ));
    }
}
