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

        let input_map = InputMap::new([
            (MoveUp, KeyCode::KeyW),
            (MoveUp, KeyCode::ArrowUp),
            (MoveDown, KeyCode::KeyS),
            (MoveDown, KeyCode::ArrowDown),
            (MoveLeft, KeyCode::KeyA),
            (MoveLeft, KeyCode::ArrowLeft),
            (MoveRight, KeyCode::KeyD),
            (MoveRight, KeyCode::ArrowRight),
        ]);

        // TODO: Figure out why gamepad bindings keeps causes a panic!
        // input_map.insert_multiple([
        //     (MoveUp, GamepadButtonType::DPadUp),
        //     (MoveDown, GamepadButtonType::DPadDown),
        //     (MoveLeft, GamepadButtonType::DPadLeft),
        //     (MoveRight, GamepadButtonType::DPadRight),
        // ]);
        // input_map.insert_axis(
        //     MoveUp,
        //     GamepadControlAxis::RIGHT_Y.with_deadzone_symmetric(0.4),
        // );
        // input_map.insert_axis(
        //     MoveDown,
        //     GamepadControlAxis::RIGHT_Y.with_deadzone_symmetric(-0.4),
        // );
        // input_map.insert_axis(
        //     MoveLeft,
        //     GamepadControlAxis::LEFT_X.with_deadzone_symmetric(-0.4),
        // );
        // input_map.insert_axis(
        //     MoveRight,
        //     GamepadControlAxis::LEFT_X.with_deadzone_symmetric(0.4),
        // );

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

        if action_state.pressed(&move_crab_left) {
            commands.entity(entity).insert(Force::Negative);
        } else if action_state.pressed(&move_crab_right) {
            commands.entity(entity).insert(Force::Positive);
        } else {
            commands.entity(entity).remove::<Force>();
        }
    }
}
