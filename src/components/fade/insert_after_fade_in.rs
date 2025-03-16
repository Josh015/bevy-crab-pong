use std::marker::PhantomData;

use bevy::prelude::*;

use crate::system_sets::StopWhenPausedSet;

use super::{Collider, Fade, Motion};

pub(super) struct InsertAfterFadeInPlugin;

impl Plugin for InsertAfterFadeInPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                insert_component_after_fading_in::<Motion>,
                insert_component_after_fading_in::<Collider>,
            )
                .in_set(StopWhenPausedSet),
        );
    }
}

/// Inserts a component after a fade-in finishes.
#[derive(Clone, Component, Copy, Debug, Default, PartialEq)]
pub struct InsertAfterFadeIn<B: Bundle + Default>(PhantomData<B>);

fn insert_component_after_fading_in<B: Bundle + Default>(
    mut commands: Commands,
    mut removed: RemovedComponents<Fade>,
    query: Query<Entity, With<InsertAfterFadeIn<B>>>,
) {
    // No need to exclude Fade::Out since the entity is already despawned.
    for entity in removed.read() {
        if query.contains(entity) {
            commands.entity(entity).insert(B::default());
        }
    }
}
