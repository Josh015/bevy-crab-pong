use bevy::prelude::*;

use crate::{
    movement::{Force, Movement},
    object::crab::Crab,
};

use super::PlayerSet;

/// Marks a [`Crab`] entity as being controlled by the input devices.
#[derive(Component, Debug)]
pub struct PlayerInput;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            move_crabs_based_on_user_input.in_set(PlayerSet),
        );
    }
}

fn move_crabs_based_on_user_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    crabs_query: Query<Entity, (With<PlayerInput>, With<Crab>, With<Movement>)>,
) {
    // Makes a Crab entity move left/right in response to the
    // keyboard's corresponding arrows keys.
    for entity in &crabs_query {
        if keyboard_input.pressed(KeyCode::Left)
            || keyboard_input.pressed(KeyCode::A)
        {
            commands.entity(entity).insert(Force::Negative);
        } else if keyboard_input.pressed(KeyCode::Right)
            || keyboard_input.pressed(KeyCode::D)
        {
            commands.entity(entity).insert(Force::Positive);
        } else {
            commands.entity(entity).remove::<Force>();
        };
    }

    // TODO: Need to make inputs account for side!
}
