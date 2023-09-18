use bevy::{prelude::*, reflect::TypePath};
use leafwing_input_manager::prelude::*;

use crate::{
    common::movement::{Force, Movement},
    level::side::Side,
    object::crab::Crab,
};

use super::CrabSet;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, TypePath)]
pub enum CrabAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

/// Marks a [`Crab`] entity as being controlled by user input devices.
#[derive(Bundle)]
pub struct CrabInputBundle {
    pub input_manager_bundle: InputManagerBundle<CrabAction>,
}

impl Default for CrabInputBundle {
    fn default() -> Self {
        use CrabAction::*;
        use GamepadAxisType::*;
        use GamepadButtonType::*;
        use KeyCode::*;

        let mut input_map = InputMap::new([
            (W, MoveUp),
            (Up, MoveUp),
            (S, MoveDown),
            (Down, MoveDown),
            (A, MoveLeft),
            (Left, MoveLeft),
            (D, MoveRight),
            (Right, MoveRight),
        ]);
        input_map.insert_multiple([
            (DPadUp, MoveUp),
            (DPadDown, MoveDown),
            (DPadLeft, MoveLeft),
            (DPadRight, MoveRight),
        ]);
        input_map.insert_multiple([
            (SingleAxis::positive_only(RightStickY, 0.4), MoveUp),
            (SingleAxis::negative_only(RightStickY, -0.4), MoveDown),
            (SingleAxis::negative_only(LeftStickX, -0.4), MoveLeft),
            (SingleAxis::positive_only(LeftStickX, 0.4), MoveRight),
        ]);

        Self {
            input_manager_bundle: InputManagerBundle::<CrabAction> {
                action_state: ActionState::default(),
                input_map,
            },
        }
    }
}

pub(super) struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<CrabAction>::default())
            .add_systems(
                Update,
                move_crabs_based_on_user_input.in_set(CrabSet),
            );
    }
}

fn move_crabs_based_on_user_input(
    mut commands: Commands,
    crabs_query: Query<
        (Entity, &ActionState<CrabAction>, &Side),
        (With<Crab>, With<Movement>),
    >,
) {
    use CrabAction::*;
    use Side::*;

    for (entity, action_state, side) in &crabs_query {
        match *side {
            Bottom => {
                if action_state.pressed(MoveLeft) {
                    commands.entity(entity).insert(Force::Negative);
                } else if action_state.pressed(MoveRight) {
                    commands.entity(entity).insert(Force::Positive);
                } else {
                    commands.entity(entity).remove::<Force>();
                }
            },
            Right => {
                if action_state.pressed(MoveUp) {
                    commands.entity(entity).insert(Force::Positive);
                } else if action_state.pressed(MoveDown) {
                    commands.entity(entity).insert(Force::Negative);
                } else {
                    commands.entity(entity).remove::<Force>();
                }
            },
            Top => {
                if action_state.pressed(MoveLeft) {
                    commands.entity(entity).insert(Force::Positive);
                } else if action_state.pressed(MoveRight) {
                    commands.entity(entity).insert(Force::Negative);
                } else {
                    commands.entity(entity).remove::<Force>();
                }
            },
            Left => {
                if action_state.pressed(MoveUp) {
                    commands.entity(entity).insert(Force::Negative);
                } else if action_state.pressed(MoveDown) {
                    commands.entity(entity).insert(Force::Positive);
                } else {
                    commands.entity(entity).remove::<Force>();
                }
            },
        }
    }
}
