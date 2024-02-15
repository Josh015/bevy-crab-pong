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
        (Entity, &Side, &ActionState<CrabAction>),
        (With<Crab>, With<Movement>),
    >,
) {
    use CrabAction::*;
    use Force::*;
    use Side::*;

    for (entity, side, action_state) in &crabs_query {
        let (left, right) = match *side {
            Bottom => ((MoveLeft, Negative), (MoveRight, Positive)),
            Right => ((MoveUp, Positive), (MoveDown, Negative)),
            Top => ((MoveLeft, Positive), (MoveRight, Negative)),
            Left => ((MoveUp, Negative), (MoveDown, Positive)),
        };

        if action_state.pressed(left.0) {
            commands.entity(entity).insert(left.1);
        } else if action_state.pressed(right.0) {
            commands.entity(entity).insert(right.1);
        } else {
            commands.entity(entity).remove::<Force>();
        }
    }
}
