use bevy::prelude::*;

use crate::GameState;

pub mod animated_water;
pub mod arena;
pub mod ball;
pub mod barrier;
pub mod enemy;
pub mod fade;
pub mod goal;
pub mod paddle;
pub mod player;
pub mod score;
pub mod swaying_camera;
pub mod velocity;
pub mod wall;

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(score::display_scores_system)
            .add_system(swaying_camera::swaying_system)
            .add_system(animated_water::animation_system)
            .add_system(fade::start_fade_system)
            .add_system(fade::step_fade_system)
            .add_system(paddle::step_fade_animation_system)
            .add_system(wall::start_fade_system)
            .add_system(wall::step_fade_animation_system)
            .add_system(ball::step_fade_animation_system)
            .add_system(goal::eliminated_animation_system)
            .add_event::<goal::GoalEliminated>()
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(paddle::movement_system)
                    .with_system(player::paddle_control_system)
                    .with_system(enemy::paddle_control_system)
                    .with_system(velocity::movement_system)
                    .with_system(goal::scored_system)
                    .with_system(goal::gameover_check_system)
                    .with_system(arena::reset_ball_position_system)
                    .with_system(arena::reset_ball_velocity_system)
                    .with_system(arena::collision_system),
            );
    }
}
