use crate::prelude::*;

mod animated_water;
mod field;
mod goals;
mod swaying_camera;
mod ui;

pub use animated_water::*;
pub use field::*;
pub use goals::*;
pub use swaying_camera::*;
pub use ui::*;

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AnimatedWaterPlugin,
            FieldPlugin,
            GoalsPlugin,
            SwayingCameraPlugin,
            UiPlugin,
        ));
    }
}
