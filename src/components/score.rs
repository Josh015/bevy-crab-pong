use bevy::{
    prelude::{Component, Query, Res, With},
    text::Text,
};

use crate::Game;

use super::goal::Goal;

#[derive(Component)]
pub struct Score;

pub fn display_scores_system(
    game: Res<Game>,
    mut query: Query<(&mut Text, &Goal), With<Score>>,
) {
    for (mut text, goal) in query.iter_mut() {
        let score_value = game.scores[&goal];
        text.sections[0].value = score_value.to_string();
    }
}
