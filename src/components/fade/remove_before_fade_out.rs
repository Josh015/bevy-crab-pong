use std::marker::PhantomData;

use bevy::prelude::*;

use crate::system_sets::StopWhenPausedSet;

use super::{Collider, Fade, Motion};

pub(super) struct RemoveBeforeFadeOutPlugin;

impl Plugin for RemoveBeforeFadeOutPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                remove_component_before_fading_out::<Motion>,
                remove_component_before_fading_out::<Collider>,
            )
                .in_set(StopWhenPausedSet),
        );
    }
}

// Removes a component before a fade-out starts.
#[derive(Clone, Component, Copy, Debug, Default, PartialEq)]
pub struct RemoveBeforeFadeOut<B: Bundle>(PhantomData<B>);

fn remove_component_before_fading_out<B: Bundle>(
    mut commands: Commands,
    query: Query<(Entity, &Fade), (With<RemoveBeforeFadeOut<B>>, Added<Fade>)>,
) {
    for (entity, fade) in &query {
        if *fade == Fade::Out {
            commands.entity(entity).remove::<B>();
        }
    }
}
