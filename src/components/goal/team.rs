use bevy::prelude::*;

use super::{Goal, HitPoints};

/// Team ID used to check for win conditions based on [`HitPoints`] value.
#[derive(Component, Debug, Default)]
#[require(Goal, HitPoints)]
pub struct Team(pub usize);

/// The team that won the previous round.
#[derive(Debug, Default, Resource)]
pub struct WinningTeam(pub usize);
