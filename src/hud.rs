use bevy::prelude::*;

use crate::{
    paddle::{HitPoints, Paddle},
    resources::GameAssets,
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
    let paddle_configs = [
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

    for (side, style) in &paddle_configs {
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
