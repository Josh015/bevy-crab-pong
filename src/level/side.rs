use bevy::prelude::*;
use serde::Deserialize;
use spew::prelude::*;
use strum::EnumIter;

use crate::{
    common::{
        collider::{Collider, ColliderSet, ColliderShapeCircle},
        fade::Fade,
        movement::Movement,
    },
    game::GameSet,
    object::{ball::Ball, crab::Crab, pole::Pole, Object},
};

pub const SIDE_WIDTH: f32 = 1.0;

/// Signals that a side has been scored in by a ball.
#[derive(Clone, Component, Debug, Event)]
pub struct SideScoredEvent(pub Side);

/// Signals that a side has been eliminated from the game.
#[derive(Clone, Component, Debug, Event)]
pub struct SideEliminatedEvent(pub Side);

/// Marks an entity that can be used as a parent to spawn [`Side`] entities.
#[derive(Component, Debug)]
pub struct SideSpawnPoint;

/// Assigns an entity to a given side of the beach.
#[derive(
    Clone, Component, Copy, Debug, Deserialize, EnumIter, Eq, Hash, PartialEq,
)]
pub enum Side {
    Bottom = 0,
    Right = 1,
    Top = 2,
    Left = 3,
}

impl Side {
    /// Perpendicular distance from a given side to a ball's center.
    ///
    /// Positive distances for inside the beach, negative for out of bounds.
    pub fn distance_to_ball(&self, ball_transform: &GlobalTransform) -> f32 {
        let ball_translation = ball_transform.translation();

        match *self {
            Self::Bottom => (0.5 * SIDE_WIDTH) - ball_translation.z,
            Self::Right => (0.5 * SIDE_WIDTH) - ball_translation.x,
            Self::Top => (0.5 * SIDE_WIDTH) + ball_translation.z,
            Self::Left => (0.5 * SIDE_WIDTH) + ball_translation.x,
        }
    }

    /// Get the normalized (+/-)(X/Z) axis the side occupies.
    pub fn axis(&self) -> Vec3 {
        match *self {
            Self::Bottom => Vec3::Z,
            Self::Right => Vec3::X,
            Self::Top => -Vec3::Z,
            Self::Left => -Vec3::X,
        }
    }

    /// Map a ball's global position to a side's local x-axis.
    pub fn get_ball_position(&self, ball_transform: &GlobalTransform) -> f32 {
        match *self {
            Self::Bottom => ball_transform.translation().x,
            Self::Right => -ball_transform.translation().z,
            Self::Top => -ball_transform.translation().x,
            Self::Left => ball_transform.translation().z,
        }
    }
}

pub(super) struct SidePlugin;

impl Plugin for SidePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SideScoredEvent>()
            .add_event::<SideEliminatedEvent>()
            .add_systems(
                Update,
                allow_only_one_crab_or_pole_per_side.after(SpewSystemSet),
            )
            .add_systems(
                PostUpdate,
                (
                    check_if_any_balls_have_scored_in_any_sides
                        .after(ColliderSet),
                    block_eliminated_sides_with_poles.after(GameSet),
                ),
            );
    }
}

fn allow_only_one_crab_or_pole_per_side(
    mut commands: Commands,
    new_query: Query<(Entity, &Side), Or<(Added<Crab>, Added<Pole>)>>,
    old_query: Query<(Entity, &Side), Or<(With<Crab>, With<Pole>)>>,
) {
    for (new_entity, new_side) in &new_query {
        for (old_entity, old_side) in &old_query {
            if old_side == new_side && old_entity != new_entity {
                commands.entity(old_entity).insert(Fade::out_default());
                break;
            }
        }
    }
}

fn check_if_any_balls_have_scored_in_any_sides(
    mut commands: Commands,
    mut side_scored_events: EventWriter<SideScoredEvent>,
    balls_query: Query<
        (Entity, &GlobalTransform, &ColliderShapeCircle),
        (With<Ball>, With<Movement>, With<Collider>),
    >,
    crabs_query: Query<&Side, (With<Crab>, With<Movement>, With<Collider>)>,
) {
    // If a ball passes a side's alive crab then despawn it and raise an event.
    for (ball_entity, global_transform, ball_collider) in &balls_query {
        for side in &crabs_query {
            let ball_distance = side.distance_to_ball(global_transform);

            if ball_distance <= ball_collider.radius {
                commands.entity(ball_entity).insert(Fade::out_default());
                side_scored_events.send(SideScoredEvent(*side));
                info!("Ball({ball_entity:?}): Scored Side({side:?})");
            }
        }
    }
}

fn block_eliminated_sides_with_poles(
    mut side_eliminated_events: EventReader<SideEliminatedEvent>,
    mut spawn_on_side_events: EventWriter<SpawnEvent<Object, Side>>,
) {
    for SideEliminatedEvent(side) in side_eliminated_events.read() {
        spawn_on_side_events.send(SpawnEvent::with_data(Object::Pole, *side));
        info!("Side({side:?}): Eliminated");
    }
}
