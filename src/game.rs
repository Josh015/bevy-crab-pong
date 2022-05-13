use crate::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

pub const ARENA_WIDTH: f32 = 1.0;
pub const ARENA_HALF_WIDTH: f32 = 0.5 * ARENA_WIDTH;
pub const ARENA_CENTER_POINT: Vec3 = Vec3::ZERO;

/// The current state of the game.
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Playing,
    GameOver,
}

/// All global information for this game.
#[derive(Default)]
pub struct Game {
    pub scores: HashMap<Goal, u32>,
    pub over: Option<GameOver>,
}

/// Game settings read from a `*.ron` config file.
#[derive(Debug, Deserialize)]
pub struct GameConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub gameover_win_message: String,
    pub gameover_lose_message: String,
    pub new_game_message: String,
    pub clear_color: Color,
    pub swaying_camera_speed: f32,
    pub animated_water_speed: f32,
    pub paddle_max_speed: f32,
    pub paddle_seconds_to_max_speed: f32,
    pub ball_starting_speed: f32,
    pub ball_max_speed: f32,
    pub ball_seconds_to_max_speed: f32,
    pub fade_speed: f32,
    pub starting_score: u32,
}

/// Represents whether the player won or lost the last game.
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum GameOver {
    Won,
    Lost,
}

impl Default for GameOver {
    fn default() -> Self {
        Self::Won
    }
}

/// When entering the gameover screen shows the corresponding UI and says
/// whether the player won/lost.
pub fn show_gameover_ui(
    config: Res<GameConfig>,
    game: Res<Game>,
    mut query: Query<&mut Text, With<GameoverMessage>>,
) {
    let mut text = query.single_mut();
    let mut message = String::new();

    // Win/Lose-specific message.
    if let Some(game_over) = game.over {
        if game_over == GameOver::Won {
            message.push_str(&config.gameover_win_message);
        } else {
            message.push_str(&config.gameover_lose_message);
        }
    }

    // General new game message.
    message.push_str(&config.new_game_message);

    text.sections[0].value = message;
}

/// Handles keyboard inputs and launching a new game when on the gameover
/// screen.
pub fn gameover_keyboard_system(
    mut state: ResMut<State<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        state.set(GameState::Playing).unwrap();
    }
}

/// Hides the gameover UI at the start of a new game.
pub fn hide_gameover_ui(mut query: Query<&mut Text, With<GameoverMessage>>) {
    let mut text = query.single_mut();
    text.sections[0].value = "".to_owned();
}

/// Resets the state of all the goals and their scores when starting a new game.
pub fn reset_game_entities(
    mut commands: Commands,
    config: Res<GameConfig>,
    mut game: ResMut<Game>,
    query: Query<(Entity, &Goal), With<Paddle>>,
    mut paddles_query: Query<
        (Entity, &mut Transform),
        (With<Paddle>, Without<Active>),
    >,
    walls_query: Query<Entity, (With<Wall>, With<Active>)>,
) {
    // Assign players
    for (entity, goal) in query.iter() {
        // TODO: Build this out to support more crazy configurations that can be
        // set at runtime
        if *goal == Goal::Bottom {
            commands.entity(entity).insert(Player);
        } else {
            commands.entity(entity).insert(Enemy);
        }
    }

    // Reset paddles
    for (entity, mut transform) in paddles_query.iter_mut() {
        transform.translation = Paddle::START_POSITION;
        commands.entity(entity).insert(Fade::In(0.4));
    }

    // Reset walls
    for entity in walls_query.iter() {
        commands.entity(entity).insert(Fade::Out(0.3));
    }

    // Reset scores
    for (_, score) in game.scores.iter_mut() {
        *score = config.starting_score;
    }

    info!("New Game");
}

/// When a goal is eliminated it checks if the current scores of all the goals
/// have triggered a gameover.
pub fn gameover_check_system(
    mut game: ResMut<Game>,
    mut state: ResMut<State<GameState>>,
    mut goal_eliminated_reader: EventReader<GoalEliminated>,
    enemies_query: Query<&Goal, (With<Paddle>, With<Enemy>)>,
    players_query: Query<&Goal, (With<Paddle>, With<Player>)>,
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
            info!("Game Over -> Player {:?}", game.over.unwrap());
        }
    }
}

/// Fades out and deactivates any `Ball` entities that are still in play at the
/// beginning of a gameover.
pub fn fade_out_balls(
    mut commands: Commands,
    query: Query<(Entity, Option<&Fade>, Option<&Active>), With<Ball>>,
) {
    for (entity, fade, active) in query.iter() {
        match fade {
            Some(Fade::In(weight)) => {
                commands.entity(entity).insert(Fade::Out(1.0 - weight));
            },
            None if active.is_some() => {
                commands.entity(entity).insert(Fade::Out(0.0));
            },
            _ => {},
        }
    }
}
