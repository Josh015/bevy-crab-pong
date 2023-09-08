use crate::{
    cached_assets::GameCachedAssets,
    components::{fading::ForState, goals::Side, paddles::HitPointsUi},
    events::MessageUiEvent,
    state::GameState,
    system_sets::GameSystemSet,
};
use bevy::prelude::*;

fn handle_spawn_ui_message_event(
    game_cached_assets: Res<GameCachedAssets>,
    mut commands: Commands,
    mut event_reader: EventReader<MessageUiEvent>,
) {
    for MessageUiEvent {
        message,
        game_screen,
    } in event_reader.iter()
    {
        commands
            .spawn((
                ForState {
                    states: vec![*game_screen],
                },
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
                                    font: game_cached_assets
                                        .font_handle
                                        .clone(),
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
    game_state: Res<GameState>,
    mut query: Query<(&Side, &mut Text), With<HitPointsUi>>,
) {
    for (side, mut text) in &mut query {
        let hit_points = game_state.goals_hit_points[side];
        text.sections[0].value = hit_points.to_string();
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
