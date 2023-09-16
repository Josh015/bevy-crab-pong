use bevy::prelude::*;
use spew::prelude::*;

use crate::{
    common::{
        collider::{Collider, ColliderSet},
        fade::Fade,
        movement::Movement,
    },
    game::{CompetitorEliminatedEvent, GameSet},
    level::side::Side,
    object::{
        ball::{Ball, BALL_RADIUS},
        crab::{Crab, CRAB_DEPTH},
        wall::Wall,
        Object,
    },
};

pub const GOAL_WIDTH: f32 = 1.0;

/// Marks a goal entity so that crabs and walls can use it as a parent, and
/// so balls can score against it.
#[derive(Component, Debug)]
pub struct Goal;

/// Signals that a goal has been scored in by a ball.
#[derive(Clone, Component, Debug, Event)]
pub struct GoalScoredEvent(pub Side);

pub struct GoalPlugin;

impl Plugin for GoalPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GoalScoredEvent>()
            .add_systems(
                Update,
                allow_only_one_crab_or_wall_per_goal.after(SpewSystemSet),
            )
            .add_systems(
                PostUpdate,
                (
                    check_if_any_balls_have_scored_in_any_goals
                        .after(ColliderSet),
                    block_eliminated_goals_with_walls.after(GameSet),
                ),
            );
    }
}

fn allow_only_one_crab_or_wall_per_goal(
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

fn check_if_any_balls_have_scored_in_any_goals(
    mut commands: Commands,
    mut goal_scored_events: EventWriter<GoalScoredEvent>,
    balls_query: Query<
        (Entity, &GlobalTransform),
        (With<Ball>, With<Movement>, With<Collider>),
    >,
    crabs_query: Query<&Side, (With<Crab>, With<Movement>, With<Collider>)>,
) {
    // If a ball passes a goal's alive crab then despawn it and raise an event.
    for (ball_entity, global_transform) in &balls_query {
        for side in &crabs_query {
            let ball_distance = side.distance_to_ball(global_transform);

            if ball_distance <= BALL_RADIUS - (0.5 * CRAB_DEPTH) {
                commands.entity(ball_entity).insert(Fade::out_default());
                goal_scored_events.send(GoalScoredEvent(*side));
                info!("Ball({:?}): Scored Goal({:?})", ball_entity, side);
            }
        }
    }
}

fn block_eliminated_goals_with_walls(
    mut competitor_eliminated_events: EventReader<CompetitorEliminatedEvent>,
    mut spawn_on_side_events: EventWriter<SpawnEvent<Object, Side>>,
) {
    for CompetitorEliminatedEvent(side) in competitor_eliminated_events.iter() {
        spawn_on_side_events.send(SpawnEvent::with_data(Object::Wall, *side));
        info!("Goal({:?}): Eliminated", side);
    }
}
