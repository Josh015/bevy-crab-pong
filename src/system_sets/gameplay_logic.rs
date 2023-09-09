use bevy::prelude::*;
use spew::prelude::SpawnEvent;

use crate::{
    components::{
        balls::*,
        goals::{Goal, Side},
        movement::*,
        paddles::*,
        spawning::*,
    },
    constants::*,
    events::{GoalEliminatedEvent, Object},
    global_data::{GameOver, GlobalData},
    screens::GameScreen,
    system_sets::GameSystemSet,
};

fn replace_despawned_balls(
    mut removed: RemovedComponents<Ball>,
    mut spawn_events: EventWriter<SpawnEvent<Object>>,
) {
    for (i, _) in removed.iter().enumerate() {
        spawn_events.send(
            SpawnEvent::new(Object::Ball)
                .delay_seconds(i as f32 * BALL_SPAWN_DELAY_IN_SECONDS),
        );
    }
}

fn handle_keyboard_input_for_player_controlled_paddles(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    paddles_query: Query<
        Entity,
        (
            With<Paddle>,
            With<KeyboardInput>,
            Without<Spawning>,
            Without<Despawning>,
        ),
    >,
) {
    // Makes a Paddle entity move left/right in response to the
    // keyboard's corresponding arrows keys.
    for entity in &paddles_query {
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
    paddles_query: Query<
        (Entity, &Side),
        (
            With<Paddle>,
            With<AiInput>,
            Without<Spawning>,
            Without<Despawning>,
        ),
    >,
    balls_query: Query<
        (Entity, &GlobalTransform),
        (With<Ball>, Without<Spawning>, Without<Despawning>),
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
        (
            With<Paddle>,
            With<AiInput>,
            Without<Spawning>,
            Without<Despawning>,
        ),
    >,
    balls_query: Query<
        &GlobalTransform,
        (With<Ball>, Without<Spawning>, Without<Despawning>),
    >,
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
    mut goal_eliminated_writer: EventWriter<GoalEliminatedEvent>,
    balls_query: Query<
        (Entity, &GlobalTransform),
        (With<Ball>, Without<Spawning>, Without<Despawning>),
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
            commands.entity(ball_entity).insert(Despawning::default());
            break;
        }
    }
}

fn block_eliminated_goals(
    mut event_reader: EventReader<GoalEliminatedEvent>,
    mut spawn_in_goal_events: EventWriter<SpawnEvent<Object, Side>>,
) {
    for GoalEliminatedEvent(eliminated_side) in event_reader.iter() {
        spawn_in_goal_events
            .send(SpawnEvent::with_data(Object::Wall, *eliminated_side));
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
                replace_despawned_balls,
                handle_keyboard_input_for_player_controlled_paddles,
                make_ai_paddles_target_the_balls_closest_to_their_goals,
                move_ai_paddles_toward_where_their_targeted_balls_will_enter_their_goals,
                check_if_any_balls_have_scored_against_any_goals,
                block_eliminated_goals,
                check_for_game_over_conditions,
            )
                .chain()
                .in_set(GameSystemSet::GameplayLogic),
        );
    }
}
