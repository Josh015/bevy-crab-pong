use bevy::prelude::*;
use std::marker::PhantomData;

use crate::game::state::PausableSet;

use super::{collider::Collider, fade::Fade, movement::Movement};

/// Inserts a component after a fade-in finishes.
#[derive(Clone, Component, Copy, Debug, Default, PartialEq)]
pub struct DelayedInsert<B: Bundle + Default>(PhantomData<B>);

// Removes a component before a fade-out starts.
#[derive(Clone, Component, Copy, Debug, Default, PartialEq)]
pub struct DelayedRemove<B: Bundle>(PhantomData<B>);

pub(super) struct DelayedPlugin;

impl Plugin for DelayedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                insert_component_after_fading_in::<Movement>,
                remove_component_before_fading_out::<Movement>,
                insert_component_after_fading_in::<Collider>,
                remove_component_before_fading_out::<Collider>,
            )
                .in_set(PausableSet),
        );
    }
}

fn insert_component_after_fading_in<B: Bundle + Default>(
    mut commands: Commands,
    mut removed: RemovedComponents<Fade>,
    query: Query<Entity, With<DelayedInsert<B>>>,
) {
    for entity in removed.read() {
        if query.contains(entity) {
            commands.entity(entity).insert(B::default());
        }
    }
}

fn remove_component_before_fading_out<B: Bundle>(
    mut commands: Commands,
    query: Query<(Entity, &Fade), (With<DelayedRemove<B>>, Added<Fade>)>,
) {
    for (entity, fade) in &query {
        if matches!(fade, Fade::Out(_)) {
            commands.entity(entity).remove::<B>();
        }
    }
}
