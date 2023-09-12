use bevy::prelude::*;

use crate::{
    paddle::{HitPoints, Paddle},
    side::Side,
    state::AppState,
};

/// Marks a [`Text`] entity to display the HP for the associated [`HitPoints`].
#[derive(Component, Debug)]
pub struct HitPointsUi;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_goal_hit_points_ui
                .chain()
                .run_if(not(in_state(AppState::Loading))),
        );
    }
}

fn update_goal_hit_points_ui(
    paddles_query: Query<(&HitPoints, &Side), With<Paddle>>,
    mut hp_ui_query: Query<(&mut Text, &Side), With<HitPointsUi>>,
) {
    for (hit_points, paddle_side) in &paddles_query {
        for (mut text, text_side) in &mut hp_ui_query {
            if text_side == paddle_side {
                text.sections[0].value = hit_points.0.to_string();
            }
        }
    }
}
