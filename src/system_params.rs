use bevy::{
    ecs::{query::QueryEntityError, system::SystemParam},
    prelude::*,
};
use derive_getters::Getters;
use std::ops::Add;

use crate::{
    assets::{GameAssets, GameMode},
    components::{Direction, Goal},
    spawners::Beach,
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
    beach: Res<'w, Beach>,
    goals_query: Query<'w, 's, &'static GlobalTransform, With<Goal>>,
}

impl Goals<'_, '_> {
    /// Get the relevant data for the corresponding [Goal] entity.
    pub fn get(&self, entity: Entity) -> Result<GoalData, QueryEntityError> {
        let global_transform = self.goals_query.get(entity)?;

        Ok(GoalData {
            beach_width: self.beach.width,
            forward: *global_transform.forward(),
            right: *global_transform.right(),
        })
    }
}

/// Data and methods related to goal logic.
#[derive(Getters)]
pub struct GoalData {
    #[getter(copy)]
    beach_width: f32,

    #[getter(copy)]
    forward: Vec3,

    #[getter(copy)]
    right: Vec3,
}

impl GoalData {
    /// Gets the entity's x position in the goal's local coordinate space.
    pub fn map_to_local_x(&self, global_transform: &GlobalTransform) -> f32 {
        global_transform.translation().dot(self.right)
    }

    /// Get the perpendicular distance from the goal to the entity.
    pub fn distance_to(&self, global_transform: &GlobalTransform) -> f32 {
        (0.5 * self.beach_width)
            + global_transform.translation().dot(self.forward)
    }

    /// Check if an entity is facing the goal.
    pub fn is_facing(&self, direction: &Direction) -> bool {
        direction.0.dot(self.forward) <= 0.0
    }
}
