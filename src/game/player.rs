use bevy::{
    input::Input,
    prelude::{Component, KeyCode, Query, Res, With},
};

use super::{
    fade::Active,
    paddle::{Movement, Paddle},
};

#[derive(Component)]
pub struct Player;

pub fn paddle_control_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Paddle, (With<Active>, With<Player>)>,
) {
    for mut paddle in query.iter_mut() {
        paddle.movement = if keyboard_input.pressed(KeyCode::Left) {
            Movement::Accelerating(-1.0)
        } else if keyboard_input.pressed(KeyCode::Right) {
            Movement::Accelerating(1.0)
        } else {
            Movement::Decelerating
        };
    }
}
