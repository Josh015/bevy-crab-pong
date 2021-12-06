use super::*;
use bevy::{
    input::Input,
    prelude::{Component, KeyCode, Query, Res, With},
};

#[derive(Component)]
pub struct Player;

pub fn paddle_control_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, (With<Paddle>, With<Active>, With<Player>)>,
) {
    for mut velocity in query.iter_mut() {
        velocity.delta = if keyboard_input.pressed(KeyCode::Left) {
            Delta::Accelerating(-1.0)
        } else if keyboard_input.pressed(KeyCode::Right) {
            Delta::Accelerating(1.0)
        } else {
            Delta::Decelerating
        };
    }
}
