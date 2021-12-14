use super::*;

/// An event fired when a `Goal` has been eliminated from play after its score
/// has reached zero.
pub struct GoalEliminated(pub Goal);

/// A component for marking `Paddle` and `Wall` entities as belonging to the
/// same goal for a given side of the arena.
#[derive(Clone, Component, Copy, Eq, PartialEq, Debug, Hash)]
pub enum Goal {
    Top,
    Right,
    Bottom,
    Left,
}

impl Goal {
    /// Perpendicular distance from a given goal to a ball's edge.
    ///
    /// Positive distances for inside the arena, negative for out of bounds.
    pub fn distance_to_ball(&self, ball_transform: &GlobalTransform) -> f32 {
        let ball_translation = ball_transform.translation;

        match *self {
            Self::Top => ARENA_HALF_WIDTH + ball_translation.z - BALL_RADIUS,
            Self::Right => ARENA_HALF_WIDTH - ball_translation.x - BALL_RADIUS,
            Self::Bottom => ARENA_HALF_WIDTH - ball_translation.z - BALL_RADIUS,
            Self::Left => ARENA_HALF_WIDTH + ball_translation.x - BALL_RADIUS,
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

    /// Map a ball's global position to a paddle's local x-axis.
    pub fn map_ball_position_to_paddle_range(
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

/// Fades out the `Paddle` and fades in the `Wall` for this `Goal` when it's
/// eliminated from play.
pub fn eliminated_system(
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
