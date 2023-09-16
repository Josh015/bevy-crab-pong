use bevy::prelude::*;

use crate::{
    barrier::BARRIER_RADIUS,
    goal::GOAL_WIDTH,
    movement::{Force, Movement, MovementSet, Speed, StoppingDistance},
};

pub const CRAB_WIDTH: f32 = 0.2;
pub const CRAB_DEPTH: f32 = 0.1;
pub const CRAB_START_POSITION: Vec3 = Vec3::new(0.0, 0.05, 0.0);
pub const CRAB_POSITION_X_MAX: f32 =
    (0.5 * GOAL_WIDTH) - BARRIER_RADIUS - (0.5 * CRAB_WIDTH);

/// Makes a crab entity that can deflect balls and move sideways inside a goal.
#[derive(Component, Debug)]
pub struct Crab;

pub struct CrabPlugin;

impl Plugin for CrabPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            restrict_crab_movement_range.after(MovementSet),
        );
    }
}

fn restrict_crab_movement_range(
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut Transform, &mut Speed, &mut StoppingDistance),
        (With<Crab>, With<Movement>),
    >,
) {
    for (entity, mut transform, mut speed, mut stopping_distance) in &mut query
    {
        // Limit crab to bounds of the goal.
        if !(-CRAB_POSITION_X_MAX..=CRAB_POSITION_X_MAX)
            .contains(&transform.translation.x)
        {
            transform.translation.x = transform
                .translation
                .x
                .clamp(-CRAB_POSITION_X_MAX, CRAB_POSITION_X_MAX);
            speed.0 = 0.0;
            commands.entity(entity).remove::<Force>();
        }

        // Limit stopping distance to the bounds of the goal.
        let stopped_position = transform.translation.x + stopping_distance.0;

        if !(-CRAB_POSITION_X_MAX..=CRAB_POSITION_X_MAX)
            .contains(&stopped_position)
        {
            stopping_distance.0 = stopped_position.signum()
                * CRAB_POSITION_X_MAX
                - transform.translation.x;
        }
    }
}
