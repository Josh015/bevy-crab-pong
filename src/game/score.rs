use super::*;
use crate::Game;
use bevy::{
    prelude::{Component, Query, Res, With},
    text::Text,
};

/// A component for marking a `Text` UI entity as displaying the score for an
/// associated `Goal`.
#[derive(Component)]
pub struct Score;

/// Updates a `Text` entity to display the current score of its associated
/// `Goal`.
pub fn update_scores_system(
    game: Res<Game>,
    mut query: Query<(&mut Text, &Goal), With<Score>>,
) {
    for (mut text, goal) in query.iter_mut() {
        let score_value = game.scores[&goal];
        text.sections[0].value = score_value.to_string();
    }
}
