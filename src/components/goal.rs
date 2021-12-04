use bevy::{ecs::prelude::*, prelude::*};

use crate::{Game, GameConfig, GameOver, GameState};

use super::{
    ball::Ball,
    enemy::Enemy,
    fade::{Active, Fade},
    paddle::Paddle,
    player::Player,
    velocity::Velocity,
    wall::Wall,
};

#[derive(Clone, Component, Copy, Eq, PartialEq, Debug, Hash)]
pub enum Goal {
    Top,
    Right,
    Bottom,
    Left,
}

pub(super) struct GoalEliminated(pub Goal);

pub(super) fn scored_system(
    mut commands: Commands,
    config: Res<GameConfig>,
    mut game: ResMut<Game>,
    mut goal_eliminated_writer: EventWriter<GoalEliminated>,
    balls_query: Query<
        (Entity, &GlobalTransform, &Velocity),
        (With<Ball>, With<Active>, Without<Fade>),
    >,
    walls_query: Query<(&GlobalTransform, &Goal), With<Wall>>,
) {
    for (entity, ball_transform, velocity) in balls_query.iter() {
        let ball_translation = ball_transform.translation;
        let ball_radius = config.ball_radius();
        let d = velocity.0.normalize();
        let radius_position = ball_translation + d * ball_radius;

        for (wall_transform, goal) in walls_query.iter() {
            // Score against the goal that's closest to this ball
            let goal_position = wall_transform.translation;
            let distance_to_goal = match goal {
                Goal::Top => radius_position.z - goal_position.z,
                Goal::Right => -radius_position.x + goal_position.x,
                Goal::Bottom => -radius_position.z + goal_position.z,
                Goal::Left => radius_position.x - goal_position.x,
            };

            if distance_to_goal > 0.0 {
                continue;
            }

            // Decrement the score and potentially eliminate the goal
            let score = game.scores.get_mut(goal).unwrap();
            *score = score.saturating_sub(1);

            if *score == 0 {
                goal_eliminated_writer.send(GoalEliminated(*goal))
            }

            // Fade out and deactivate the ball to prevent repeated scoring
            commands.entity(entity).insert(Fade::Out(0.0));
            break;
        }
    }
}

pub(super) fn eliminated_animation_system(
    mut commands: Commands,
    mut goal_eliminated_reader: EventReader<GoalEliminated>,
    balls_query: Query<(Entity, &Goal), (With<Paddle>, With<Active>)>,
    walls_query: Query<(Entity, &Goal), (With<Wall>, Without<Active>)>,
) {
    for GoalEliminated(eliminated_goal) in goal_eliminated_reader.iter() {
        for (entity, goal) in balls_query.iter() {
            if goal == eliminated_goal {
                commands.entity(entity).insert(Fade::Out(0.0));
                break;
            }
        }

        for (entity, goal) in walls_query.iter() {
            if goal == eliminated_goal {
                commands.entity(entity).insert(Fade::In(0.0));
                break;
            }
        }
    }
}

pub(super) fn gameover_check_system(
    mut game: ResMut<Game>,
    mut state: ResMut<State<GameState>>,
    mut goal_eliminated_reader: EventReader<GoalEliminated>,
    players_query: Query<&Goal, (With<Paddle>, With<Player>)>,
    enemies_query: Query<&Goal, (With<Paddle>, With<Enemy>)>,
) {
    for GoalEliminated(_) in goal_eliminated_reader.iter() {
        // Player wins if all Enemy paddles have a score of zero
        let has_player_won =
            enemies_query.iter().all(|goal| game.scores[&goal] == 0);

        // Player loses if all Player paddles have a score of zero
        let has_player_lost =
            players_query.iter().all(|goal| game.scores[&goal] == 0);

        // Declare a winner and trigger gameover
        if has_player_won || has_player_lost {
            game.over = if has_player_won {
                Some(GameOver::Won)
            } else {
                Some(GameOver::Lost)
            };

            state.set(GameState::GameOver).unwrap();
        }
    }
}
