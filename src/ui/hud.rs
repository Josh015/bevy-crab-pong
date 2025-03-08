use bevy::prelude::*;

use crate::{
    components::side::Side,
    game::{
        assets::GameAssets,
        competitors::Competitors,
        state::{GameState, LoadedSet},
    },
};

pub(super) struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Loading), spawn_hud_ui)
            .add_systems(
                Update,
                update_hit_points_ui.chain().in_set(LoadedSet),
            );
    }
}

/// Marks a [`Text`] entity to display the HP for an associated [`Side`].
#[derive(Component, Debug)]
pub struct HitPointsUi;

fn spawn_hud_ui(game_assets: Res<GameAssets>, mut commands: Commands) {
    let hp_ui_configs = [
        (
            Side::Bottom,
            Node {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                bottom: Val::Px(5.0),
                right: Val::Px(400.0),
                ..default()
            },
        ),
        (
            Side::Right,
            Node {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                top: Val::Px(400.0),
                right: Val::Px(5.0),
                ..default()
            },
        ),
        (
            Side::Top,
            Node {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                top: Val::Px(5.0),
                left: Val::Px(400.0),
                ..default()
            },
        ),
        (
            Side::Left,
            Node {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                bottom: Val::Px(400.0),
                left: Val::Px(5.0),
                ..default()
            },
        ),
    ];

    for (side, node) in &hp_ui_configs {
        commands.spawn((
            HitPointsUi,
            *side,
            node.clone(),
            Text("0".to_string()),
            TextFont {
                font: game_assets.font_menu.clone(),
                font_size: 50.0,
                ..Default::default()
            },
            TextColor(Srgba::RED.into()),
        ));
    }
}

fn update_hit_points_ui(
    competitors: Res<Competitors>,
    mut hp_ui_query: Query<(&mut Text, &Side), With<HitPointsUi>>,
) {
    for (mut text, side) in &mut hp_ui_query {
        let competitor = &competitors.0[side];

        text.0 = competitor.hit_points.to_string();
    }
}
