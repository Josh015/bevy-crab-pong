use bevy::prelude::*;
use bevy_ui_anchor::{AnchorTarget, AnchorUiNode};

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

/// Marks a [`Text`] entity to display the HP for an associated [`HitPoints`]
/// entity.
#[derive(Component, Debug)]
pub struct HitPointsUi;

fn update_hit_points_ui(
    hp_query: Query<&HitPoints, With<Goal>>,
    mut hp_ui_query: Query<(&mut Text, &AnchorUiNode), With<HitPointsUi>>,
) {
    for (mut text, anchor) in &mut hp_ui_query {
        let AnchorTarget::Entity(entity) = anchor.target else {
            continue;
        };
        let Ok(hp) = hp_query.get(entity) else {
            continue;
        };

        text.0 = hp.0.to_string();
    }
}
