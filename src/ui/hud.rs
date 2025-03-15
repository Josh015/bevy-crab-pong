use bevy::prelude::*;

use crate::{
    components::{Goal, HitPoints},
    game::LoadedSet,
};

pub(super) struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_hit_points_ui.chain().in_set(LoadedSet));
    }
}

/// Marks a [`Text`] entity to display the value of an associated [`HitPoints`]
/// entity.
#[derive(Component, Debug)]
#[require(Text)]
pub struct HitPointsUiSource(pub Entity);

fn update_hit_points_ui(
    hp_query: Query<&HitPoints, With<Goal>>,
    mut hp_ui_query: Query<(&mut Text, &HitPointsUiSource)>,
) {
    for (mut text, source) in &mut hp_ui_query {
        let Ok(hp) = hp_query.get(source.0) else {
            continue;
        };

        text.0 = hp.0.to_string();
    }
}
