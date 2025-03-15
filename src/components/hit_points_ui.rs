use bevy::prelude::*;

use crate::{
    components::{Goal, HitPoints},
    LoadedSet,
};

pub(super) struct HitPointsUiPlugin;

impl Plugin for HitPointsUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_hit_points_ui.chain().in_set(LoadedSet));
    }
}

/// Marks a [`Text`] entity to display the value of an associated [`HitPoints`]
/// entity.
#[derive(Component, Debug)]
#[require(Text)]
pub struct HitPointsUi {
    pub goal_entity: Entity,
}

fn update_hit_points_ui(
    hp_query: Query<&HitPoints, With<Goal>>,
    mut hp_ui_query: Query<(&mut Text, &HitPointsUi)>,
) {
    for (mut text, source) in &mut hp_ui_query {
        let Ok(hp) = hp_query.get(source.goal_entity) else {
            continue;
        };

        text.0 = hp.0.to_string();
    }
}
