use bevy::prelude::*;

use crate::collider::ColliderSet;

pub const DEBUGGING_RAY_LENGTH: f32 = 20.0;

/// Toggles displaying debugging gizmos.
#[derive(Debug, Default, Resource)]
pub struct IsDebuggingMode(pub bool);

#[derive(SystemSet, Clone, Hash, Debug, PartialEq, Eq)]
pub struct DebugModeSet;

pub struct DebugModePlugin;

impl Plugin for DebugModePlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            PostUpdate,
            DebugModeSet
                .after(ColliderSet)
                .run_if(show_debugging_gizmos),
        )
        .init_resource::<IsDebuggingMode>();
    }
}

fn show_debugging_gizmos(is_debugging_mode: Res<IsDebuggingMode>) -> bool {
    is_debugging_mode.0
}
