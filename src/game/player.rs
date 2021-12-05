use bevy::{
    input::Input,
    prelude::{Component, KeyCode, Query, Res, With},
};

use super::{fade::Active, paddle::Paddle};

#[derive(Component)]
pub struct Player;

pub fn paddle_control_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Paddle, (With<Active>, With<Player>)>,
) {
    for mut paddle in query.iter_mut() {
        *paddle = if keyboard_input.pressed(KeyCode::Left) {
            Paddle::Left
        } else if keyboard_input.pressed(KeyCode::Right) {
            Paddle::Right
        } else {
            Paddle::Stop
        };
    }
}
