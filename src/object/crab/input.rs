use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    common::movement::{Force, Movement},
    game::state::PlayableSet,
    level::side::Side,
};

use super::Crab;

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
                move_crabs_based_on_user_input.in_set(PlayableSet),
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
    use Side::*;

    for (entity, side, action_state) in &crabs_query {
        let (move_crab_left, move_crab_right) = match *side {
            Bottom => (MoveLeft, MoveRight),
            Right => (MoveDown, MoveUp),
            Top => (MoveRight, MoveLeft),
            Left => (MoveUp, MoveDown),
        };

        if action_state.pressed(move_crab_left) {
            commands.entity(entity).insert(Force::Negative);
        } else if action_state.pressed(move_crab_right) {
            commands.entity(entity).insert(Force::Positive);
        } else {
            commands.entity(entity).remove::<Force>();
        }
    }
}
