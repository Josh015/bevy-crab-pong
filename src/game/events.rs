use bevy::prelude::*;

use crate::components::side::Side;

pub(super) struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SideScoredEvent>()
            .add_event::<SideEliminatedEvent>();
    }
}

/// Signals that a side has been scored in by a ball.
#[derive(Clone, Debug, Event)]
pub struct SideScoredEvent(pub Side);

/// Signals that a side has been eliminated from the game.
#[derive(Clone, Debug, Event)]
pub struct SideEliminatedEvent(pub Side);
