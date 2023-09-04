#![allow(clippy::type_complexity)]

use crate::prelude::*;

fn show_debug_gizmos(run_state: Res<RunState>) -> bool {
    run_state.has_debug_gizmos
}

fn debug_ball_path_gizmos(
    query: Query<(&GlobalTransform, &Heading), (With<Ball>, Without<Fade>)>,
    mut gizmos: Gizmos,
) {
    for (global_transform, heading) in &query {
        gizmos.line(
            global_transform.translation(),
            global_transform.translation() + heading.0 * 20.0,
            Color::RED,
        )
        // TODO: Draw a sphere over the goal position where the ball is expected
        // to cross.
    }
}

pub struct GizmosPlugin;

impl Plugin for GizmosPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            debug_ball_path_gizmos.run_if(show_debug_gizmos),
        );
    }
}
