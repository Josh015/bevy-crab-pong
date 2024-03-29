pub mod collider;
pub mod fade;
pub mod movement;

use bevy::prelude::*;

pub(super) struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            collider::ColliderPlugin,
            fade::FadePlugin,
            movement::MovementPlugin,
        ));
    }
}
