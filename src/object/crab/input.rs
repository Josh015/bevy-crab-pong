use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    common::movement::{Force, Movement},
    level::side::Side,
};

use super::{Crab, CrabSet};

#[derive(Actionlike, Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect)]
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
        let (left_action, left_force, right_action, right_force) = match *side {
            Bottom => (MoveLeft, Force::Negative, MoveRight, Force::Positive),
            Right => (MoveUp, Force::Positive, MoveDown, Force::Negative),
            Top => (MoveLeft, Force::Positive, MoveRight, Force::Negative),
            Left => (MoveUp, Force::Negative, MoveDown, Force::Positive),
        };

        if action_state.pressed(left_action) {
            commands.entity(entity).insert(left_force);
        } else if action_state.pressed(right_action) {
            commands.entity(entity).insert(right_force);
        } else {
            commands.entity(entity).remove::<Force>();
        }
    }
}
