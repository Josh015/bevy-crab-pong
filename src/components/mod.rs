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

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameSystemSet {
    Debugging,
    GameplayLogic,
    Movement,
    Collision,
    Despawning,
}

fn show_debugging_systems(run_state: Res<RunState>) -> bool {
    run_state.is_debugging_enabled
}

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            Update,
            GameSystemSet::Debugging
                .before(GameSystemSet::Movement)
                .run_if(show_debugging_systems)
                .run_if(not(in_state(GameScreen::StartMenu))),
        );
        app.configure_set(
            Update,
            GameSystemSet::GameplayLogic
                .before(GameSystemSet::Movement)
                .run_if(in_state(GameScreen::Playing)),
        );
        app.configure_set(
            Update,
            GameSystemSet::Movement.run_if(not(in_state(GameScreen::Paused))),
        );
        app.configure_set(
            PostUpdate,
            GameSystemSet::Collision
                .after(GameSystemSet::Movement)
                .run_if(not(in_state(GameScreen::Paused))),
        );
        app.configure_set(
            PostUpdate,
            GameSystemSet::Despawning
                .after(GameSystemSet::Collision)
                .run_if(not(in_state(GameScreen::Paused))),
        );
        app.add_plugins((
            AnimatedWaterPlugin,
            BallPlugin,
            ColliderPlugin,
            AiPlugin,
            FadePlugin,
            ForStatePlugin,
            GoalPlugin,
            MovementPlugin,
            PaddlePlugin,
            PlayerPlugin,
            SwayingCameraPlugin,
            UiPlugin,
            WallPlugin,
        ));
    }
}
