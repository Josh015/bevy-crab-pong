use bevy::prelude::*;
use serde::Deserialize;
use strum::EnumIter;

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
