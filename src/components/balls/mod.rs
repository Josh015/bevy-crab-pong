use crate::prelude::*;

mod ball;
mod collider;

pub use ball::*;
pub use collider::*;

pub struct BallsPlugin;

impl Plugin for BallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((BallPlugin, ColliderPlugin));
    }
}
