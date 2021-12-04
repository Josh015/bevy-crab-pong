use bevy::{
    input::Input,
    prelude::{Component, KeyCode, Query, Res, With},
};

use super::{
    crab::{Crab, Movement},
    fade::Active,
};

#[derive(Component)]
pub struct Player;

pub fn crab_control_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Crab, (With<Active>, With<Player>)>,
) {
    for mut crab in query.iter_mut() {
        crab.movement = if keyboard_input.pressed(KeyCode::Left) {
            Movement::Left
        } else if keyboard_input.pressed(KeyCode::Right) {
            Movement::Right
        } else {
            Movement::Stopped
        };
    }
}
