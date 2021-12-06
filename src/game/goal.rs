use super::*;
use bevy::{ecs::prelude::*, prelude::*};

#[derive(Clone, Component, Copy, Eq, PartialEq, Debug, Hash)]
pub enum Goal {
    Top,
    Right,
    Bottom,
    Left,
}

impl Goal {
    /// Perpendicular distance from a ball's edge to a given goal. Positive
    /// distances for inside the arena, negative for out of bounds.
    pub fn distance_to_ball(
        &self,
        config: &Res<GameConfig>,
        ball_transform: &GlobalTransform,
    ) -> f32 {
        let ball_radius = config.ball_radius();
        let half_width = 0.5 * config.beach_width;
        let ball_translation = ball_transform.translation;

        match *self {
            Self::Top => half_width + ball_translation.z - ball_radius,
            Self::Right => half_width - ball_translation.x - ball_radius,
            Self::Bottom => half_width - ball_translation.z - ball_radius,
            Self::Left => half_width + ball_translation.x - ball_radius,
        }
    }

    /// Get the (+/-)(X/Z) axis the goal occupies.
    pub fn axis(&self) -> Vec3 {
        match *self {
            Self::Top => -Vec3::Z,
            Self::Right => Vec3::X,
            Self::Bottom => Vec3::Z,
            Self::Left => -Vec3::X,
        }
    }

    /// Map the ball's global position to a paddle's local x-axis.
    pub fn map_ball_to_paddle_axis(
        &self,
        ball_transform: &GlobalTransform,
    ) -> f32 {
        match *self {
            Self::Top => -ball_transform.translation.x,
            Self::Right => -ball_transform.translation.z,
            Self::Bottom => ball_transform.translation.x,
            Self::Left => ball_transform.translation.z,
        }
    }
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
