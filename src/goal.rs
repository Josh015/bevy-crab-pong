use bevy::prelude::*;
use spew::prelude::*;
use std::ops::RangeInclusive;

use crate::{
    ball::Ball,
    barrier::BARRIER_RADIUS,
    collider::{Collider, ColliderSet},
    crab::{Crab, CRAB_HALF_DEPTH, CRAB_HALF_WIDTH},
    fade::Fade,
    game::Game,
    movement::Movement,
    object::Object,
    side::Side,
    wall::Wall,
};

pub const GOAL_WIDTH: f32 = 1.0;
pub const GOAL_HALF_WIDTH: f32 = 0.5 * GOAL_WIDTH;
pub const GOAL_CRAB_START_POSITION: Vec3 = Vec3::new(0.0, 0.05, 0.0);
pub const GOAL_CRAB_MAX_POSITION_X: f32 =
    GOAL_HALF_WIDTH - BARRIER_RADIUS - CRAB_HALF_WIDTH;
pub const GOAL_CRAB_MAX_POSITION_RANGE: RangeInclusive<f32> =
    -GOAL_CRAB_MAX_POSITION_X..=GOAL_CRAB_MAX_POSITION_X;

/// Marks a goal entity so that crabs and walls can use it as a parent, and
/// so balls can score against it.
#[derive(Component, Debug)]
pub struct Goal;

/// Signals a goal being eliminated from the game.
#[derive(Clone, Component, Debug, Event)]
pub struct GoalEliminatedEvent(pub Entity);

pub struct GoalPlugin;

impl Plugin for GoalPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GoalEliminatedEvent>()
            .add_systems(
                Update,
                allow_only_one_crab_or_wall_in_a_goal.after(SpewSystemSet),
            )
            .add_systems(
                PostUpdate,
                (
                    goal_scored_and_potentially_eliminated,
                    block_eliminated_goals_with_walls,
                )
                    .chain()
                    .after(ColliderSet),
            );
    }
}

fn allow_only_one_crab_or_wall_in_a_goal(
    mut commands: Commands,
    new_query: Query<(Entity, &Parent), Or<(Added<Crab>, Added<Wall>)>>,
    old_query: Query<(Entity, &Parent), Or<(With<Crab>, With<Wall>)>>,
) {
    for (new_entity, new_parent) in &new_query {
        for (old_entity, old_parent) in &old_query {
            if old_parent == new_parent && old_entity != new_entity {
                commands
                    .entity(old_entity)
                    .remove::<Movement>()
                    .insert(Fade::out_default());
                break;
            }
        }
    }
}

fn goal_scored_and_potentially_eliminated(
    mut commands: Commands,
    mut goal_eliminated_events: EventWriter<GoalEliminatedEvent>,
    balls_query: Query<
        (Entity, &GlobalTransform),
        (With<Ball>, With<Movement>, With<Collider>),
    >,
    mut goals_query: Query<(Entity, &Side), With<Goal>>,
    mut game: ResMut<Game>,
) {
    for (ball_entity, global_transform) in &balls_query {
        for (entity, side) in &mut goals_query {
            // A ball will score against the goal it's closest to once it's
            // fully past the goal's crab.
            let ball_distance = side.distance_to_ball(global_transform);

            if ball_distance > -CRAB_HALF_DEPTH {
                continue;
            }

            // Decrement the crab's HP and potentially eliminate it.
            let Some(competitor) = game.competitors.get_mut(side) else {
                continue;
            };
            competitor.hit_points = competitor.hit_points.saturating_sub(1);
            info!("Ball({:?}): Scored Goal({:?})", ball_entity, side);

            if competitor.hit_points == 0 {
                goal_eliminated_events.send(GoalEliminatedEvent(entity));
                info!("Ball({:?}): Eliminated Goal({:?})", ball_entity, side);
            }

            // Despawn and replace the scoring ball.
            commands.entity(ball_entity).insert(Fade::out_default());
            break;
        }
    }
}

fn block_eliminated_goals_with_walls(
    mut goal_eliminated_events: EventReader<GoalEliminatedEvent>,
    mut spawn_in_goal_events: EventWriter<SpawnEvent<Object, Entity>>,
) {
    for GoalEliminatedEvent(entity) in goal_eliminated_events.iter() {
        spawn_in_goal_events.send(SpawnEvent::with_data(Object::Wall, *entity));
    }
}
