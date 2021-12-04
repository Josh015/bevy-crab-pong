mod files;
mod game;

use bevy::{ecs::prelude::*, prelude::*};
use game::*;

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
        .add_startup_system(setup)
        .add_system(animated_water::animation_system)
        .add_system(ball::step_fade_animation_system)
        .add_system(fade::start_fade_system)
        .add_system(fade::step_fade_system)
        .add_system(goal::eliminated_animation_system)
        .add_system(paddle::step_fade_animation_system)
        .add_system(score::update_scores_system)
        .add_system(swaying_camera::swaying_system)
        .add_system(wall::start_fade_system)
        .add_system(wall::step_fade_animation_system)
        .add_system_set(
            SystemSet::on_enter(GameState::GameOver)
                .with_system(show_gameover_ui),
        )
        .add_system_set(
            SystemSet::on_update(GameState::GameOver)
                .with_system(gameover_keyboard_system),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::GameOver)
                .with_system(hide_gameover_ui),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(reset_game_entities),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(ball::collision_system)
                .with_system(ball::reset_position_system)
                .with_system(ball::reset_velocity_system)
                .with_system(enemy::paddle_control_system)
                .with_system(goal::gameover_check_system)
                .with_system(goal::scored_system)
                .with_system(paddle::movement_system)
                .with_system(player::paddle_control_system)
                .with_system(velocity::movement_system),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Playing).with_system(fade_out_balls),
        )
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}
