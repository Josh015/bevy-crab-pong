use crate::{
    cached_assets::CachedAssets,
    components::{
        balls::*,
        fading::*,
        goals::{Goal, Side},
        movement::*,
        paddles::*,
    },
    config::Config,
    constants::*,
    events::*,
    global_data::{GameOver, GlobalData},
    screens::GameScreen,
    system_sets::GameSystemSet,
};
use bevy::prelude::*;
use rand::prelude::*;

fn spawn_balls_as_needed_from_the_center_of_the_arena(
    global_data: Res<GlobalData>,
    config: Res<Config>,
    cached_assets: Res<CachedAssets>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    new_balls_query: Query<
        (Entity, Option<&Fade>),
        (With<Ball>, Without<Heading>, Without<Speed>),
    >,
    all_balls_query: Query<&Ball>,
) {
    // Check for any non-moving new balls.
    for (entity, fade) in &new_balls_query {
        // Pause the spawning process until the new ball finishes fading in.
        if fade.is_some() {
            return;
        }

        // Make the ball collidable and launch it in a random direction.
        let mut rng = SmallRng::from_entropy();
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);

        commands.entity(entity).insert((
            Collider,
            VelocityBundle {
                heading: Heading(Vec3::new(angle.cos(), 0.0, angle.sin())),
                speed: Speed(config.ball_speed),
            },
        ));
        info!("Ball({:?}): Launched", entity);
    }

    // Spawn new balls until max is reached.
    if all_balls_query.iter().count()
        >= config.modes[global_data.mode_index].max_ball_count
    {
        return;
    }

    let entity = commands
        .spawn((
            Ball,
            ForState {
                states: vec![GameScreen::Playing, GameScreen::Paused],
            },
            FadeBundle::default(),
            PbrBundle {
                mesh: cached_assets.ball_mesh.clone(),
                material: materials.add(StandardMaterial {
                    alpha_mode: AlphaMode::Blend,
                    base_color: Color::rgba(1.0, 1.0, 1.0, 0.0),
                    ..default()
                }),
                transform: Transform::from_matrix(
                    Mat4::from_scale_rotation_translation(
                        Vec3::splat(BALL_DIAMETER),
                        Quat::IDENTITY,
                        BALL_SPAWNER_POSITION,
                    ),
                ),
                ..default()
            },
        ))
        .id();

    info!("Ball({:?}): Spawning", entity);
}

fn handle_keyboard_input_for_player_controlled_paddles(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    query: Query<Entity, (With<KeyboardInput>, With<Paddle>)>,
) {
    // Makes a Paddle entity move left/right in response to the
    // keyboard's corresponding arrows keys.
    for entity in &query {
        if keyboard_input.pressed(KeyCode::Left)
            || keyboard_input.pressed(KeyCode::A)
        {
            commands.entity(entity).insert(Force::Negative);
        } else if keyboard_input.pressed(KeyCode::Right)
            || keyboard_input.pressed(KeyCode::D)
        {
            commands.entity(entity).insert(Force::Positive);
        } else {
            commands.entity(entity).remove::<Force>();
        };
    }

    // TODO: Need to make inputs account for side!
}

fn make_ai_paddles_target_the_balls_closest_to_their_goals(
    mut commands: Commands,
    paddles_query: Query<(Entity, &Side), (With<AiInput>, With<Paddle>)>,
    balls_query: Query<
        (Entity, &GlobalTransform),
        (With<Ball>, With<Collider>),
    >,
) {
    for (paddle_entity, side) in &paddles_query {
        let mut closest_ball_distance = std::f32::MAX;
        let mut target = None;

        for (ball_entity, ball_transform) in &balls_query {
            let ball_distance_to_goal = side.distance_to_ball(ball_transform);

            if ball_distance_to_goal < closest_ball_distance {
                closest_ball_distance = ball_distance_to_goal;
                target = Some(ball_entity);
            }
        }

        if let Some(target) = target {
            commands.entity(paddle_entity).insert(Target(target));
        } else {
            commands.entity(paddle_entity).remove::<Target>();
        }
    }
}

fn move_ai_paddles_toward_where_their_targeted_balls_will_enter_their_goals(
    mut commands: Commands,
    paddles_query: Query<
        (
            Entity,
            &Side,
            &Transform,
            &StoppingDistance,
            Option<&Target>,
        ),
        (With<AiInput>, With<Paddle>),
    >,
    balls_query: Query<&GlobalTransform, (With<Ball>, With<Collider>)>,
) {
    for (entity, side, transform, stopping_distance, target) in &paddles_query {
        // Use the ball's goal position or default to the center of the goal.
        let mut target_goal_position = FIELD_CENTER_POINT.x;

        if let Some(target) = target {
            if let Ok(ball_transform) = balls_query.get(target.0) {
                target_goal_position = side.get_ball_position(ball_transform)
            }
        }

        // Move the paddle so that its center is over the target goal position.
        let paddle_stop_position =
            transform.translation.x + stopping_distance.0;
        let distance_from_paddle_center =
            (paddle_stop_position - target_goal_position).abs();

        if distance_from_paddle_center
            < PADDLE_CENTER_HIT_AREA_PERCENTAGE * PADDLE_HALF_WIDTH
        {
            commands.entity(entity).remove::<Force>();
        } else {
            commands.entity(entity).insert(
                if target_goal_position < transform.translation.x {
                    Force::Negative // Left
                } else {
                    Force::Positive // Right
                },
            );
        }
    }
}

fn check_if_any_balls_have_scored_against_any_goals(
    mut commands: Commands,
    mut global_data: ResMut<GlobalData>,
    mut fade_out_entity_events: EventWriter<FadeOutEntityEvent>,
    mut goal_eliminated_writer: EventWriter<GoalEliminatedEvent>,
    balls_query: Query<
        (Entity, &GlobalTransform),
        (With<Ball>, With<Collider>),
    >,
    goals_query: Query<&Side, With<Goal>>,
) {
    for (ball_entity, global_transform) in &balls_query {
        for side in &goals_query {
            // A ball will score against the goal it's closest to once it's
            // fully past the goal's paddle.
            let ball_distance = side.distance_to_ball(global_transform);

            if ball_distance > -PADDLE_HALF_DEPTH {
                continue;
            }

            // Decrement the goal's HP and potentially eliminate it.
            let hit_points =
                global_data.goals_hit_points.get_mut(side).unwrap();

            *hit_points = hit_points.saturating_sub(1);
            info!("Ball({:?}): Scored Goal({:?})", ball_entity, side);

            if *hit_points == 0 {
                goal_eliminated_writer.send(GoalEliminatedEvent(*side));
                info!("Ball({:?}): Eliminated Goal({:?})", ball_entity, side);
            }

            // Remove Collider and start fading out the ball to prevent
            // repeated scoring.
            commands.entity(ball_entity).remove::<Collider>();
            fade_out_entity_events.send(FadeOutEntityEvent(ball_entity));
            break;
        }
    }
}

/// Disables a given [`Goal`] to remove it from play.
fn handle_goal_eliminated_event(
    mut commands: Commands,
    mut event_reader: EventReader<GoalEliminatedEvent>,
    mut fade_out_entity_events: EventWriter<FadeOutEntityEvent>,
    mut spawn_wall_events: EventWriter<SpawnWallEvent>,
    paddles_query: Query<
        (Entity, &Side),
        (With<Paddle>, With<Collider>, Without<Fade>),
    >,
) {
    for GoalEliminatedEvent(eliminated_side) in event_reader.iter() {
        // Fade out the paddle for the eliminated goal.
        for (paddle_entity, side) in &paddles_query {
            if *side != *eliminated_side {
                continue;
            }

            // Stop the paddle from moving and colliding.
            commands
                .entity(paddle_entity)
                .remove::<(Collider, VelocityBundle)>();
            fade_out_entity_events.send(FadeOutEntityEvent(paddle_entity));
            break;
        }

        // Fade in the wall for the eliminated goal.
        spawn_wall_events.send(SpawnWallEvent {
            side: *eliminated_side,
            is_instant: false,
        });
    }
}

fn check_for_game_over_conditions(
    mut global_data: ResMut<GlobalData>,
    mut next_game_screen: ResMut<NextState<GameScreen>>,
    mut event_reader: EventReader<GoalEliminatedEvent>,
    teams_query: Query<(&Team, &Side), With<Paddle>>,
) {
    // TODO: Need a more robust system that allows 4 teams!
    for GoalEliminatedEvent(_) in event_reader.iter() {
        // See if player or enemies have lost enough paddles for a game over.
        let has_player_won = teams_query
            .iter()
            .filter(|(team, _)| **team == Team::Enemies)
            .all(|(_, side)| global_data.goals_hit_points[side] == 0);

        let has_player_lost = teams_query
            .iter()
            .filter(|(team, _)| **team == Team::Allies)
            .all(|(_, side)| global_data.goals_hit_points[side] == 0);

        if !has_player_won && !has_player_lost {
            continue;
        }

        // Declare a winner and navigate back to the Start Menu.
        global_data.game_over = Some(if has_player_won {
            GameOver::Won
        } else {
            GameOver::Lost
        });

        next_game_screen.set(GameScreen::StartMenu);
        info!("Game Over: Player {:?}", global_data.game_over.unwrap());
    }
}

// TODO: Debug option to directly control single ball's exact position with
// keyboard and see how paddles respond. Can go in goals, triggering a score and
// ball return?

pub struct GameplayLogicPlugin;

impl Plugin for GameplayLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_balls_as_needed_from_the_center_of_the_arena,
                handle_keyboard_input_for_player_controlled_paddles,
                make_ai_paddles_target_the_balls_closest_to_their_goals,
                move_ai_paddles_toward_where_their_targeted_balls_will_enter_their_goals,
                check_if_any_balls_have_scored_against_any_goals,
                handle_goal_eliminated_event,
                check_for_game_over_conditions,
            )
                .chain()
                .in_set(GameSystemSet::GameplayLogic),
        );
    }
}
