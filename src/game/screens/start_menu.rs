#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

use crate::prelude::*;

fn spawn_start_menu_ui(
    game_config: Res<GameConfig>,
    game_state: Res<GameState>,
    mut ui_message_events: EventWriter<MessageUiEvent>,
) {
    let mut message = String::from(match game_state.game_over {
        Some(GameOver::Won) => &game_config.game_over_win_message,
        Some(GameOver::Lost) => &game_config.game_over_lose_message,
        _ => "",
    });

    message.push_str(&game_config.new_game_message);

    ui_message_events.send(MessageUiEvent {
        message,
        game_screen: GameScreen::StartMenu,
    });
}

/// Resets all goal HP fields to their starting value.
fn reset_hit_points(
    game_config: Res<GameConfig>,
    mut game_state: ResMut<GameState>,
) {
    const SIDES: [Side; 4] = [Side::Top, Side::Right, Side::Bottom, Side::Left];
    let goals = &game_config.modes[game_state.mode_index].goals;

    for (i, side) in SIDES.iter().enumerate() {
        game_state
            .goals_hit_points
            .insert(*side, goals[i].starting_hit_points);
    }
}

/// Fades out any existing [`Wall`] entities.
fn despawn_walls(
    mut commands: Commands,
    mut fade_out_entity_events: EventWriter<FadeOutEntityEvent>,
    query: Query<Entity, With<Wall>>,
) {
    for entity in &query {
        commands.entity(entity).remove::<Collider>();
        fade_out_entity_events.send(FadeOutEntityEvent(entity));
    }
}

/// Spawns [`Paddle`] entities for their corresponding goals.
fn spawn_paddles(
    mut commands: Commands,
    game_state: Res<GameState>,
    game_config: Res<GameConfig>,
    resources: Res<GameCachedAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut fade_out_entity_events: EventWriter<FadeOutEntityEvent>,
    paddles_query: Query<Entity, (With<Paddle>, Without<Fade>)>,
    goals_query: Query<(Entity, &Side), With<Goal>>,
) {
    // Fade out existing paddles so new ones can spawn at starting positions.
    for entity in &paddles_query {
        commands
            .entity(entity)
            .remove::<(Collider, VelocityBundle)>();
        fade_out_entity_events.send(FadeOutEntityEvent(entity));
    }

    // Give every paddle a parent so we can use relative transforms.
    for (i, (entity, side)) in goals_query.iter().enumerate() {
        let goal_config = &game_config.modes[game_state.mode_index].goals[i];
        let material_handle = resources.paddle_material_handles[side].clone();

        commands.entity(entity).with_children(|parent| {
            let mut paddle = parent.spawn((
                *side,
                Paddle,
                Collider,
                FadeBundle {
                    fade_animation: FadeAnimation::Scale {
                        max_scale: PADDLE_SCALE,
                        axis_mask: Vec3::ONE,
                    },
                    ..default()
                },
                AccelerationBundle {
                    velocity: VelocityBundle {
                        heading: Heading(Vec3::X),
                        ..default()
                    },
                    max_speed: MaxSpeed(game_config.paddle_max_speed),
                    acceleration: Acceleration(
                        game_config.paddle_max_speed
                            / game_config.paddle_seconds_to_max_speed,
                    ),
                    ..default()
                },
                PbrBundle {
                    mesh: resources.paddle_mesh_handle.clone(),
                    material: material_handle.clone(),
                    transform: Transform::from_matrix(
                        Mat4::from_scale_rotation_translation(
                            Vec3::splat(f32::EPSILON),
                            Quat::IDENTITY,
                            GOAL_PADDLE_START_POSITION,
                        ),
                    ),
                    ..default()
                },
                if goal_config.team == TeamConfig::Enemies {
                    Team::Enemies
                } else {
                    Team::Allies
                },
            ));

            if goal_config.controlled_by == ControlledByConfig::AI {
                paddle.insert(AiInput);
            } else {
                paddle.insert(KeyboardInput);
            }

            let material = materials.get_mut(&material_handle).unwrap();
            material.base_color = Color::hex(&goal_config.color).unwrap()
        });
    }
}

pub struct StartMenuPlugin;

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameScreen::StartMenu), spawn_start_menu_ui)
            .add_systems(
                OnExit(GameScreen::StartMenu),
                (reset_hit_points, despawn_walls, spawn_paddles).chain(),
            );
    }
}