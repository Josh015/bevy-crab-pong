use bevy::{prelude::*, reflect::TypePath};
use leafwing_input_manager::prelude::*;

use crate::{
    common::movement::{Force, Movement},
    level::side::Side,
    object::crab::Crab,
};

use super::PlayerSet;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, TypePath)]
pub enum PlayerAction {
    MoveCrabUp,
    MoveCrabDown,
    MoveCrabLeft,
    MoveCrabRight,
}

/// Marks a [`Crab`] entity as being controlled by user input devices.
#[derive(Bundle)]
pub struct PlayerInputBundle {
    pub input_manager_bundle: InputManagerBundle<PlayerAction>,
}

impl Default for PlayerInputBundle {
    fn default() -> Self {
        use GamepadAxisType::*;
        use GamepadButtonType::*;
        use KeyCode::*;
        use PlayerAction::*;

        let mut input_map = InputMap::new([
            (W, MoveCrabUp),
            (Up, MoveCrabUp),
            (S, MoveCrabDown),
            (Down, MoveCrabDown),
            (A, MoveCrabLeft),
            (Left, MoveCrabLeft),
            (D, MoveCrabRight),
            (Right, MoveCrabRight),
        ]);
        input_map.insert_multiple([
            (DPadUp, MoveCrabUp),
            (DPadDown, MoveCrabDown),
            (DPadLeft, MoveCrabLeft),
            (DPadRight, MoveCrabRight),
        ]);
        input_map.insert_multiple([
            (SingleAxis::positive_only(RightStickY, 0.4), MoveCrabUp),
            (SingleAxis::negative_only(RightStickY, -0.4), MoveCrabDown),
            (SingleAxis::negative_only(LeftStickX, -0.4), MoveCrabLeft),
            (SingleAxis::positive_only(LeftStickX, 0.4), MoveCrabRight),
        ]);

        Self {
            input_manager_bundle: InputManagerBundle::<PlayerAction> {
                action_state: ActionState::default(),
                input_map,
            },
        }
    }
}

pub(super) struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(
                Update,
                move_crabs_based_on_user_input.in_set(PlayerSet),
            );
    }
}

fn move_crabs_based_on_user_input(
    mut commands: Commands,
    crabs_query: Query<
        (Entity, &ActionState<PlayerAction>, &Side),
        (With<Crab>, With<Movement>),
    >,
) {
    use PlayerAction::*;
    use Side::*;

    for (entity, action_state, side) in &crabs_query {
        match *side {
            Bottom => {
                if action_state.pressed(MoveCrabLeft) {
                    commands.entity(entity).insert(Force::Negative);
                } else if action_state.pressed(MoveCrabRight) {
                    commands.entity(entity).insert(Force::Positive);
                } else {
                    commands.entity(entity).remove::<Force>();
                }
            },
            Right => {
                if action_state.pressed(MoveCrabUp) {
                    commands.entity(entity).insert(Force::Positive);
                } else if action_state.pressed(MoveCrabDown) {
                    commands.entity(entity).insert(Force::Negative);
                } else {
                    commands.entity(entity).remove::<Force>();
                }
            },
            Top => {
                if action_state.pressed(MoveCrabLeft) {
                    commands.entity(entity).insert(Force::Positive);
                } else if action_state.pressed(MoveCrabRight) {
                    commands.entity(entity).insert(Force::Negative);
                } else {
                    commands.entity(entity).remove::<Force>();
                }
            },
            Left => {
                if action_state.pressed(MoveCrabUp) {
                    commands.entity(entity).insert(Force::Negative);
                } else if action_state.pressed(MoveCrabDown) {
                    commands.entity(entity).insert(Force::Positive);
                } else {
                    commands.entity(entity).remove::<Force>();
                }
            },
        }
    }
}
