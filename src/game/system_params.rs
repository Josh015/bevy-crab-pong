use bevy::{
    ecs::{query::QueryEntityError, system::SystemParam},
    prelude::*,
};
use std::ops::Add;

use crate::components::{goal::Goal, movement::Heading, side::Side};

use super::{
    assets::{GameAssets, GameMode},
    level::Level,
};

pub(super) struct SystemParamsPlugin;

impl Plugin for SystemParamsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedGameMode>();
    }
}

#[derive(Debug, Default, Resource)]
struct SelectedGameMode(usize);

/// Allows systems to query and set the current game mode.
#[derive(SystemParam)]
pub struct GameModes<'w> {
    game_assets: Res<'w, GameAssets>,
    game_modes: Res<'w, Assets<GameMode>>,
    selected: ResMut<'w, SelectedGameMode>,
}

impl GameModes<'_> {
    /// Gets the current game mode.
    pub fn current(&self) -> &GameMode {
        self.game_modes
            .get(&self.game_assets.game_modes[self.selected.0])
            .unwrap()
    }

    /// Switch to the previous game mode.
    pub fn previous(&mut self) {
        self.selected.0 = self.selected.0.saturating_sub(1);
    }

    /// Switch to the next game mode.
    pub fn next(&mut self) {
        self.selected.0 = self
            .selected
            .0
            .add(1)
            .min(self.game_assets.game_modes.len() - 1);
    }
}

/// Allows system to do work related to [Goal] entities.
#[derive(SystemParam)]
pub struct Goals<'w, 's> {
    level: Res<'w, Level>,
    goals_query:
        Query<'w, 's, (&'static Side, &'static GlobalTransform), With<Goal>>,
}

impl Goals<'_, '_> {
    /// Get the relevant data for the corresponding [Goal] entity.
    pub fn get(&self, entity: Entity) -> Result<GoalData, QueryEntityError> {
        let (side, global_transform) = self.goals_query.get(entity)?;

        Ok(GoalData {
            level_width: self.level.width,
            back: *global_transform.back(),
            right: *global_transform.right(),
            side: *side,
        })
    }
}

/// Data and methods related to goal logic.
pub struct GoalData {
    pub level_width: f32,
    pub back: Vec3,
    pub right: Vec3,
    pub side: Side,
}

impl GoalData {
    /// Gets the ball's x position in the goal's local coordinate space.
    pub fn map_ball_to_local_x(
        &self,
        ball_global_transform: &GlobalTransform,
    ) -> f32 {
        ball_global_transform.translation().dot(self.right)
    }

    /// Get the perpendicular distance from the goal to the ball.
    pub fn distance_to_ball(
        &self,
        ball_global_transform: &GlobalTransform,
    ) -> f32 {
        (0.5 * self.level_width)
            - ball_global_transform.translation().dot(self.back)
    }

    /// Check if a ball is facing the goal.
    pub fn has_ball_facing_it(&self, ball_heading: &Heading) -> bool {
        ball_heading.0.dot(self.back) > 0.0
    }
}
