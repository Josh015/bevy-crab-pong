use bevy::prelude::*;

use crate::{
    assets::GameAssets,
    crab::{Crab, HitPoints},
    side::Side,
    state::AppState,
};

/// Marks a [`Text`] entity to display the HP for the associated [`HitPoints`].
#[derive(Component, Debug)]
pub struct HitPointsUi;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Loading), spawn_hud_ui)
            .add_systems(
                Update,
                update_goal_hit_points_ui
                    .chain()
                    .run_if(not(in_state(AppState::Loading))),
            );
    }
}

fn spawn_hud_ui(game_assets: Res<GameAssets>, mut commands: Commands) {
    let hp_ui_configs = [
        (
            Side::Bottom,
            Style {
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
            Style {
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
            Style {
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
            Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                bottom: Val::Px(400.0),
                left: Val::Px(5.0),
                ..default()
            },
        ),
    ];

    for (side, style) in &hp_ui_configs {
        commands.spawn((
            *side,
            HitPointsUi,
            TextBundle {
                style: style.clone(),
                text: Text::from_section(
                    "0",
                    TextStyle {
                        font: game_assets.font_menu.clone(),
                        font_size: 50.0,
                        color: Color::RED,
                    },
                ),
                ..default()
            },
        ));
    }
}

fn update_goal_hit_points_ui(
    crabs_query: Query<(&HitPoints, &Side), With<Crab>>,
    mut hp_ui_query: Query<(&mut Text, &Side), With<HitPointsUi>>,
) {
    for (hit_points, crab_side) in &crabs_query {
        for (mut text, text_side) in &mut hp_ui_query {
            if text_side == crab_side {
                text.sections[0].value = hit_points.0.to_string();
            }
        }
    }
}
