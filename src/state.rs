use crate::prelude::*;
use std::collections::HashMap;

/// Current screen of the game.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum GameScreen {
    StartMenu,
    Playing,
    Paused,
}

/// Represents whether the player won or lost the last game.
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash)]
pub enum GameOver {
    #[default]
    Won,
    Lost,
}

/// All global information for this game.
#[derive(Debug, Resource)]
pub struct RunState {
    pub goals_hit_points: HashMap<Side, u32>,
    pub game_over: Option<GameOver>,

    // TODO: Move these to corresponding component files!
    pub font_handle: Handle<Font>,
    pub paddle_mesh_handle: Handle<Mesh>,
    pub paddle_material_handles: HashMap<Side, Handle<StandardMaterial>>,
}

impl FromWorld for RunState {
    fn from_world(world: &mut World) -> Self {
        let font_handle = {
            let asset_server = world.get_resource::<AssetServer>().unwrap();

            asset_server.load("fonts/FiraSans-Bold.ttf")
        };
        let paddle_mesh_handle = {
            let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();

            meshes.add(Mesh::from(shape::Cube { size: 1.0 }))
        };
        let paddle_material_handles = {
            let mut materials = world
                .get_resource_mut::<Assets<StandardMaterial>>()
                .unwrap();

            HashMap::from([
                (Side::Bottom, materials.add(Color::RED.into())),
                (Side::Right, materials.add(Color::BLUE.into())),
                (Side::Top, materials.add(Color::ORANGE.into())),
                (Side::Left, materials.add(Color::PURPLE.into())),
            ])
        };

        Self {
            goals_hit_points: HashMap::with_capacity(4),
            game_over: None,
            font_handle,
            paddle_mesh_handle,
            paddle_material_handles,
        }
    }
}

/// Resets all goal HP fields to their starting value.
pub fn reset_hit_points(
    config: Res<GameConfig>,
    mut run_state: ResMut<RunState>,
) {
    for (_, hit_points) in &mut run_state.goals_hit_points {
        *hit_points = config.starting_hit_points;
    }
}

/// Checks for conditions that would trigger a game over.
pub fn game_over_check(
    mut run_state: ResMut<RunState>,
    mut game_screen: ResMut<State<GameScreen>>,
    mut event_reader: EventReader<GoalEliminatedEvent>,
    enemies_query: Query<&Side, (With<Paddle>, With<Ai>)>,
    players_query: Query<&Side, (With<Paddle>, With<Player>)>,
) {
    for GoalEliminatedEvent(_) in event_reader.iter() {
        // See if player or enemies have lost enough paddles for a game over.
        let has_player_won = enemies_query
            .iter()
            .all(|side| run_state.goals_hit_points[side] == 0);

        let has_player_lost = players_query
            .iter()
            .all(|side| run_state.goals_hit_points[side] == 0);

        if !has_player_won && !has_player_lost {
            continue;
        }

        // Declare a winner and navigate back to the Start Menu.
        run_state.game_over = if has_player_won {
            Some(GameOver::Won)
        } else {
            Some(GameOver::Lost)
        };

        game_screen.set(GameScreen::StartMenu).unwrap();
        info!("Game Over -> Player {:?}", run_state.game_over.unwrap());
    }
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RunState>()
            .add_state(GameScreen::StartMenu)
            .add_system_set(
                SystemSet::on_exit(GameScreen::StartMenu)
                    .with_system(reset_hit_points),
            )
            .add_system_set(
                SystemSet::on_update(GameScreen::Playing)
                    .with_system(game_over_check.after(goal_eliminated_event)),
            )
            .add_startup_system(reset_hit_points);
    }
}
