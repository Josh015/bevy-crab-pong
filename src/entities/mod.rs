use crate::prelude::*;

mod arena;
mod ball;
mod inputs;
mod movement;
mod spawning;

pub use arena::*;
pub use ball::*;
pub use inputs::*;
pub use movement::*;
pub use spawning::*;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ArenaPlugin,
            BallPlugin,
            MovementPlugin,
            SpawningPlugin,
        ));
    }
}
