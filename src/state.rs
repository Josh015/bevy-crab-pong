use crate::prelude::*;
use std::collections::HashMap;

/// Current state of the app.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum AppState {
    StartMenu,
    Game,
    Pause,
}

/// Component to tag an entity as only needed in one state.
#[derive(Component)]
pub struct ForState<T> {
    pub states: Vec<T>,
}

/// When entering a new state this despawns [`ForState`] entities that aren't
/// configured for it.
pub fn app_state_enter_despawn_system(
    mut commands: Commands,
    state: Res<State<AppState>>,
    mut fade_out_entity_events: EventWriter<FadeOutEntityEvent>,
    mut query: Query<(Entity, &ForState<AppState>, Option<&mut FadeAnimation>)>,
) {
    for (entity, for_state, fade_animation) in &mut query {
        if for_state.states.contains(state.current()) {
            continue;
        }

        if fade_animation.is_some() {
            fade_out_entity_events.send(FadeOutEntityEvent {
                entity,
                is_stopped: true,
            });
        } else {
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// Represents whether the player won or lost the last game.
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash)]
pub enum GameOver {
    #[default]
    Won,
    Lost,
}

/// All global information for this game.
#[derive(Debug)]
pub struct RunState {
    pub goals_hit_points: HashMap<Side, u32>,
    pub game_over: Option<GameOver>,
    pub next_ball_material_index: usize,

    // Store the most used asset handles.
    pub font_handle: Handle<Font>,
    pub paddle_mesh_handle: Handle<Mesh>,
    pub paddle_material_handles: HashMap<Side, Handle<StandardMaterial>>,
    pub ball_mesh_handle: Handle<Mesh>,
    pub wall_mesh_handle: Handle<Mesh>,
    pub wall_material_handle: Handle<StandardMaterial>,
}

impl FromWorld for RunState {
    fn from_world(world: &mut World) -> Self {
        let font_handle = {
            let asset_server = world.get_resource::<AssetServer>().unwrap();

            asset_server.load("fonts/FiraSans-Bold.ttf")
        };
        let (wall_mesh_handle, paddle_mesh_handle, ball_mesh_handle) = {
            let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();

            (
                meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                meshes.add(Mesh::from(shape::Icosphere {
                    radius: 0.5,
                    subdivisions: 2,
                })),
            )
        };
        let (wall_material_handle, paddle_material_handles) = {
            let mut materials = world
                .get_resource_mut::<Assets<StandardMaterial>>()
                .unwrap();

            (
                materials.add(Color::hex("00A400").unwrap().into()),
                HashMap::from([
                    (Side::Bottom, materials.add(Color::RED.into())),
                    (Side::Right, materials.add(Color::BLUE.into())),
                    (Side::Top, materials.add(Color::ORANGE.into())),
                    (Side::Left, materials.add(Color::PURPLE.into())),
                ]),
            )
        };

        Self {
            goals_hit_points: HashMap::with_capacity(4),
            game_over: None,
            next_ball_material_index: 0,
            font_handle,
            paddle_mesh_handle,
            paddle_material_handles,
            ball_mesh_handle,
            wall_mesh_handle,
            wall_material_handle,
        }
    }
}

/// Resets all goal HP fields to their starting value.
pub fn reset_hit_points_system(
    config: Res<GameConfig>,
    mut run_state: ResMut<RunState>,
) {
    for (_, hit_points) in &mut run_state.goals_hit_points {
        *hit_points = config.starting_hit_points;
    }
}

/// Checks for conditions that would trigger a game over.
pub fn game_over_check_system(
    mut run_state: ResMut<RunState>,
    mut state: ResMut<State<AppState>>,
    mut event_reader: EventReader<GoalEliminatedEvent>,
    enemies_query: Query<&Side, (With<Paddle>, With<Enemy>)>,
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

        state.set(AppState::StartMenu).unwrap();
        info!("Game Over -> Player {:?}", run_state.game_over.unwrap());
    }
}
