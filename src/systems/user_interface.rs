use bevy::prelude::*;

use crate::{
    cached_assets::CachedAssets,
    components::{
        goals::Side,
        paddles::Paddle,
        scoring::{HitPoints, HitPointsUi},
        spawning::ForStates,
    },
    events::MessageUiEvent,
};

use super::GameSystemSet;

fn handle_spawn_ui_message_event(
    cached_assets: Res<CachedAssets>,
    mut commands: Commands,
    mut message_ui_events: EventReader<MessageUiEvent>,
) {
    for MessageUiEvent {
        message,
        game_screen,
    } in message_ui_events.iter()
    {
        commands
            .spawn((
                ForStates([*game_screen]),
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    background_color: Color::NONE.into(),
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            position_type: PositionType::Absolute,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle {
                            style: Style {
                                margin: UiRect::all(Val::Px(5.0)),
                                ..default()
                            },
                            text: Text::from_section(
                                message.clone(),
                                TextStyle {
                                    font: cached_assets.menu_font.clone(),
                                    font_size: 30.0,
                                    color: Color::RED,
                                },
                            ),
                            ..default()
                        });
                    });
            });
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

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_spawn_ui_message_event, update_goal_hit_points_ui)
                .chain()
                .in_set(GameSystemSet::UserInterface),
        );
    }
}
