use bevy::prelude::*;
use spew::prelude::SpawnEvent;

use crate::{
    components::{
        AiPlayer, Ball, Despawning, Force, HitPoints, KeyboardPlayer, Object,
        Paddle, Side, Spawning, StoppingDistance, Target, Team,
    },
    constants::*,
    resources::{GameAssets, GameConfig, SelectedGameMode, WinningTeam},
    states::GameState,
};

use super::GameSystemSet;

#[derive(Clone, Component, Debug, Event)]
struct GoalEliminatedEvent(Entity);

fn spawn_balls_sequentially_as_needed(
    game_assets: Res<GameAssets>,
    game_configs: Res<Assets<GameConfig>>,
    selected_mode: Res<SelectedGameMode>,
    balls_query: Query<Entity, With<Ball>>,
    spawning_balls_query: Query<Entity, (With<Ball>, With<Spawning>)>,
    mut spawn_events: EventWriter<SpawnEvent<Object>>,
) {
    let game_config = game_configs.get(&game_assets.game_config).unwrap();

    if balls_query.iter().len()
        < game_config.modes[selected_mode.0].max_ball_count
        && spawning_balls_query.iter().len() < 1
    {
        spawn_events.send(SpawnEvent::new(Object::Ball));
    }
}

fn handle_keyboard_input_for_player_controlled_paddles(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    paddles_query: Query<
        Entity,
        (
            With<Paddle>,
            With<KeyboardPlayer>,
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
            With<AiPlayer>,
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

fn move_ai_paddles_toward_their_targeted_balls(
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
            With<AiPlayer>,
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
    mut goal_eliminated_events: EventWriter<GoalEliminatedEvent>,
    balls_query: Query<
        (Entity, &GlobalTransform),
        (With<Ball>, Without<Spawning>, Without<Despawning>),
    >,
    mut paddles_query: Query<(&Parent, &mut HitPoints, &Side), With<Paddle>>,
) {
    for (ball_entity, global_transform) in &balls_query {
        for (parent, mut hit_points, side) in &mut paddles_query {
            // A ball will score against the goal it's closest to once it's
            // fully past the goal's paddle.
            let ball_distance = side.distance_to_ball(global_transform);

            if ball_distance > -PADDLE_HALF_DEPTH {
                continue;
            }

            // Decrement the paddle's HP and potentially eliminate it.
            hit_points.0 = hit_points.0.saturating_sub(1);
            info!("Ball({:?}): Scored Goal({:?})", ball_entity, side);

            if hit_points.0 == 0 {
                goal_eliminated_events.send(GoalEliminatedEvent(parent.get()));
                info!("Ball({:?}): Eliminated Goal({:?})", ball_entity, side);
            }

            // Despawn and replace the scoring ball.
            commands.entity(ball_entity).insert(Despawning);
            break;
        }
    }
}

fn block_eliminated_goals(
    mut goal_eliminated_events: EventReader<GoalEliminatedEvent>,
    mut spawn_in_goal_events: EventWriter<SpawnEvent<Object, Entity>>,
) {
    for GoalEliminatedEvent(entity) in goal_eliminated_events.iter() {
        spawn_in_goal_events.send(SpawnEvent::with_data(Object::Wall, *entity));
    }
}

fn check_for_game_over(
    mut commands: Commands,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut goal_eliminated_events: EventReader<GoalEliminatedEvent>,
    teams_query: Query<(&Team, &HitPoints), With<Paddle>>,
) {
    for GoalEliminatedEvent(_) in goal_eliminated_events.iter() {
        // Check if only one team still has HP.
        let Some((survivor, _)) = teams_query.iter().find(|(_, hp)| hp.0 > 0)
        else {
            return;
        };
        let is_winner = teams_query
            .iter()
            .all(|(team, hp)| team.0 == survivor.0 || hp.0 == 0);

        if !is_winner {
            continue;
        }

        // Declare a winner and navigate back to the Start Menu.
        commands.insert_resource(WinningTeam(survivor.0));
        next_game_state.set(GameState::StartMenu);
        info!("Game Over: Team {:?} won!", survivor.0);
    }
}

// TODO: Debug option to directly control single ball's exact position with
// keyboard and see how paddles respond. Can go in goals, triggering a score and
// ball return?

pub struct GameplayLogicPlugin;

impl Plugin for GameplayLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GoalEliminatedEvent>().add_systems(
            Update,
            (
                spawn_balls_sequentially_as_needed,
                handle_keyboard_input_for_player_controlled_paddles,
                make_ai_paddles_target_the_balls_closest_to_their_goals,
                move_ai_paddles_toward_their_targeted_balls,
                check_if_any_balls_have_scored_against_any_goals,
                block_eliminated_goals,
                check_for_game_over,
            )
                .chain()
                .in_set(GameSystemSet::GameplayLogic),
        );
    }
}
