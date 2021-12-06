use super::*;
use bevy::{
    input::Input,
    prelude::{Component, KeyCode, Query, Res, With},
};

#[derive(Component)]
pub struct Player;

pub fn paddle_control_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Movement, (With<Paddle>, With<Active>, With<Player>)>,
) {
    for mut movement in query.iter_mut() {
        movement.delta = if keyboard_input.pressed(KeyCode::Left) {
            Some(Delta::Negative)
        } else if keyboard_input.pressed(KeyCode::Right) {
            Some(Delta::Positive)
        } else {
            None
        };
    }
}
