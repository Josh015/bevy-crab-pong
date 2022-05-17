mod arena;
mod components;
mod config;
mod fade;
mod files;
mod goal;
mod movement;
mod state;
mod ui;

pub mod prelude {
    pub use crate::arena::*;
    pub use crate::components::*;
    pub use crate::config::*;
    pub use crate::fade::*;
    pub use crate::goal::*;
    pub use crate::movement::*;
    pub use crate::state::*;
    pub use crate::ui::*;
    pub use bevy::math::*;
    pub use bevy::prelude::*;
    pub use rand::prelude::*;
}

use crate::prelude::*;

fn main() {
    let config: GameConfig =
        files::load_config_from_file("assets/config/game.ron");

    App::new()
        .insert_resource(WindowDescriptor {
            title: config.title.clone(),
            width: config.width as f32,
            height: config.height as f32,
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(config.clear_color))
        .add_plugins(DefaultPlugins)
        .add_event::<MessageUiEvent>()
        .add_event::<SpawnWallEvent>()
        .add_event::<GoalScoredEvent>()
        .add_event::<GoalEliminatedEvent>()
        .insert_resource(config)
        .init_resource::<RunState>()
        .add_state(AppState::StartMenu)
        .add_system_set(
            SystemSet::on_enter(AppState::StartMenu)
                .with_system(spawn_start_menu_ui)
                .with_system(app_state_enter_despawn),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::StartMenu)
                .with_system(reset_hit_points)
                .with_system(goal_despawn_walls)
                .with_system(spawn_paddles),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::Game)
                .with_system(app_state_enter_despawn),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(arena_collision_system)
                .with_system(movement_system)
                .with_system(goal_paddle_collision_system)
                .with_system(goal_paddle_ai_control_system)
                .with_system(arena_ball_spawner_system)
                .with_system(goal_scored_check_system)
                .with_system(goal_scored_event.after(goal_scored_check_system))
                .with_system(
                    goal_eliminated_event.after(goal_scored_check_system),
                )
                .with_system(
                    game_over_check_system.after(goal_eliminated_event),
                ),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::Pause)
                .with_system(spawn_pause_ui)
                .with_system(app_state_enter_despawn),
        )
        .add_system(user_input_system)
        .add_system(spawn_ui_message_event)
        .add_system(goal_hit_points_ui_system)
        .add_system(spawn_wall_event)
        .add_system(arena_animated_water_system)
        .add_system(arena_swaying_camera_system)
        .add_system(fade_animation_system)
        .add_startup_system(spawn_arena)
        .add_startup_system(reset_hit_points.after(spawn_arena))
        .run();
}

// TODO: Need a fix for the rare occasion when a ball just bounces infinitely
// between two walls in a straight line? Maybe make all bounces slightly adjust
// ball angle rather than pure reflection?

// TODO: Offer a "Traditional" mode with two paddles (1xPlayer, 1xEnemy)
// opposite each other and the other two walled off. Also just one ball?

// TODO: Debug option to make all paddles driven by AI? Will need to revise
// player system to handle no players.

// TODO: Debug option to directly control single ball's exact position with
// keyboard and see how paddles respond. Can go in goals, triggering a score and
// ball return?

// TODO: Debug option to add small cubes at the projected points on goals with
// debug lines to the nearest ball. Also add a line from the paddle to a flat
// but wide cube (to allow both to be visible if they overlap) that matches the
// paddle's hit box dimensions and is positioned where the paddle predicts it
// will stop. One of each per goal so we can spawn them in advance.
