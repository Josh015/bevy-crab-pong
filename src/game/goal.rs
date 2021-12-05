use bevy::{ecs::prelude::*, prelude::*};

use super::{
    ball::Ball,
    enemy::Enemy,
    fade::{Active, Fade},
    paddle::Paddle,
    player::Player,
    velocity::Velocity,
    wall::Wall,
};

#[derive(Clone, Component, Copy, Eq, PartialEq, Debug, Hash)]
pub enum Goal {
    Top,
    Right,
    Bottom,
    Left,
}

pub struct GoalEliminated(pub Goal);

pub fn eliminated_animation_system(
    mut commands: Commands,
    mut goal_eliminated_reader: EventReader<GoalEliminated>,
    balls_query: Query<(Entity, &Goal), (With<Paddle>, With<Active>)>,
    walls_query: Query<(Entity, &Goal), (With<Wall>, Without<Active>)>,
) {
    for GoalEliminated(eliminated_goal) in goal_eliminated_reader.iter() {
        for (entity, goal) in balls_query.iter() {
            if goal == eliminated_goal {
                commands.entity(entity).insert(Fade::Out(0.0));
                break;
            }
        }

        for (entity, goal) in walls_query.iter() {
            if goal == eliminated_goal {
                commands.entity(entity).insert(Fade::In(0.0));
                break;
            }
        }
    }
}
