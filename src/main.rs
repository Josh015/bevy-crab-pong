mod files;
mod game;

use bevy::{ecs::prelude::*, prelude::*};
use game::*;

#[derive(Clone, Debug, Eq, PartialEq, Hash, SystemLabel)]
enum GameSystems {
    StatusChange,
    EntityLogic,
    Collisions,
    Transformations,
    Mirroring,
    GoalCheck,
}

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
        .init_resource::<Game>()
        .insert_resource(config)
        .add_state(GameState::GameOver)
        .add_event::<GoalEliminated>()
        .add_startup_system(game::setup)
        .add_system(score::update_scores_system)
        .add_system_set(
            SystemSet::new()
                .label(GameSystems::StatusChange)
                .with_system(fade::begin_fade_system)
                .with_system(wall::begin_fade_system),
        )
        .add_system_set(
            SystemSet::new()
                .label(GameSystems::Transformations)
                .after(GameSystems::Collisions)
                .with_system(animated_water::animation_system)
                .with_system(ball::fade_animation_system)
                .with_system(fade::step_fade_system)
                .with_system(paddle::fade_animation_system)
                .with_system(swaying_camera::swaying_system)
                .with_system(wall::fade_animation_system),
        )
        .add_system_set(
            SystemSet::new()
                .label(GameSystems::Mirroring)
                .after(GameSystems::Transformations)
                .with_system(mirror::reflect_parent_entities_system),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::GameOver)
                .with_system(game::show_gameover_ui),
        )
        .add_system_set(
            SystemSet::on_update(GameState::GameOver)
                .with_system(game::gameover_keyboard_system),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::GameOver)
                .with_system(game::hide_gameover_ui),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(game::reset_game_entities),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .label(GameSystems::EntityLogic)
                .after(GameSystems::StatusChange)
                .with_system(ball::inactive_ball_reset_system)
                .with_system(ball::reactivated_ball_launch_system)
                .with_system(enemy::ai_paddle_control_system)
                .with_system(player::keyboard_paddle_control_system),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .label(GameSystems::Collisions)
                .after(GameSystems::EntityLogic)
                .with_system(ball::collision_system)
                .with_system(ball::goal_scored_system)
                .with_system(paddle::bounded_movement_system)
                .with_system(paddle::stop_when_inactive_system),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .after(GameSystems::Transformations)
                .before(GameSystems::Mirroring)
                .with_system(movement::acceleration_system),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .label(GameSystems::GoalCheck)
                .after(GameSystems::Mirroring)
                .with_system(game::gameover_check_system)
                .with_system(goal::eliminated_system),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Playing)
                .with_system(game::fade_out_balls),
        )
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}
