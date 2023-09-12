use bevy::prelude::*;

use crate::{
    paddle::{HitPoints, Paddle},
    resources::GameAssets,
    side::Side,
    state::{AppState, ForStates},
};

/// Marks a [`Text`] entity to display the HP for the associated [`HitPoints`].
#[derive(Component, Debug)]
pub struct HitPointsUi;

/// An event fired when spawning a message UI.
#[derive(Event, Debug)]
pub struct MessageUiEvent {
    pub message: String,
    pub game_state: AppState,
}

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MessageUiEvent>().add_systems(
            Update,
            (handle_spawn_ui_message_event, update_goal_hit_points_ui)
                .chain()
                .run_if(not(in_state(AppState::Loading))),
        );
    }
}

fn handle_spawn_ui_message_event(
    game_assets: Res<GameAssets>,
    mut commands: Commands,
    mut message_ui_events: EventReader<MessageUiEvent>,
) {
    for MessageUiEvent {
        message,
        game_state,
    } in message_ui_events.iter()
    {
        commands
            .spawn((
                ForStates([*game_state]),
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
                                    font: game_assets.font_menu.clone(),
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
