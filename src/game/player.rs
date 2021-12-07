use super::*;
use bevy::{
    input::Input,
    prelude::{Component, KeyCode, Query, Res, With},
};

/// A component for marking a `Paddle` entity as being controller by player
/// input.
#[derive(Component)]
pub struct Player;

/// Makes a `Paddle` entity move left/right in response to the keyboard's
/// corresponding arrows keys.
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
