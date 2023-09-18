use bevy::{prelude::*, reflect::TypePath};
use leafwing_input_manager::prelude::*;

use crate::{
    common::movement::{Force, Movement},
    object::crab::Crab,
};

use super::PlayerSet;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, TypePath)]
pub enum PlayerAction {
    MoveCrabLeft,
    MoveCrabRight,
}

/// Marks a [`Crab`] entity as being controlled by the input devices.
#[derive(Component, Debug)]
pub struct PlayerInput;

pub(super) struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(
                Update,
                (
                    configure_crab_inputs,
                    move_crabs_based_on_user_input.in_set(PlayerSet),
                ),
            );
    }
}

fn configure_crab_inputs(
    mut commands: Commands,
    crabs_query: Query<Entity, Added<PlayerInput>>,
) {
    use GamepadAxisType::*;
    use GamepadButtonType::*;
    use KeyCode::*;
    use PlayerAction::*;

    for entity in &crabs_query {
        let mut input_map = InputMap::new([
            (A, MoveCrabLeft),
            (Left, MoveCrabLeft),
            (D, MoveCrabRight),
            (Right, MoveCrabRight),
        ]);
        input_map.insert_multiple([
            (DPadLeft, MoveCrabLeft),
            (DPadRight, MoveCrabRight),
        ]);
        input_map.insert_multiple([
            (SingleAxis::negative_only(LeftStickX, -0.4), MoveCrabLeft),
            (SingleAxis::positive_only(LeftStickX, 0.4), MoveCrabRight),
        ]);

        commands
            .entity(entity)
            .insert(InputManagerBundle::<PlayerAction> {
                action_state: ActionState::default(),
                input_map,
            });
    }
}

fn move_crabs_based_on_user_input(
    mut commands: Commands,
    crabs_query: Query<
        (Entity, &ActionState<PlayerAction>),
        (With<PlayerInput>, With<Crab>, With<Movement>),
    >,
) {
    use PlayerAction::*;

    for (entity, action_state) in &crabs_query {
        if action_state.pressed(MoveCrabLeft) {
            commands.entity(entity).insert(Force::Negative);
        } else if action_state.pressed(MoveCrabRight) {
            commands.entity(entity).insert(Force::Positive);
        } else {
            commands.entity(entity).remove::<Force>();
        };
    }

    // TODO: Need to make inputs account for side!
}
