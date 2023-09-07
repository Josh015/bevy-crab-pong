use crate::prelude::*;

mod barrier;
mod goal;
mod paddle;
mod side;
mod team;
mod wall;

pub use barrier::*;
pub use goal::*;
pub use paddle::*;
pub use side::*;
pub use team::*;
pub use wall::*;

pub struct GoalsPlugin;

impl Plugin for GoalsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((BarrierPlugin, GoalPlugin, PaddlePlugin, WallPlugin));
    }
}
