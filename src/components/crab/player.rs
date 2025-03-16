use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    components::{Force, Movement, Side},
    system_sets::ActiveDuringGameplaySet,
};

use super::Crab;

pub(super) struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<CrabAction>::default())
            .add_systems(
                Update,
                move_crabs_based_on_user_input.in_set(ActiveDuringGameplaySet),
            );
    }
}

/// [`Player`] input actions that move [`Crab`] entities.
#[derive(Actionlike, Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect)]
pub enum CrabAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

/// Makes a Player [`Crab`] entity.
#[derive(Component)]
#[require(ActionState<CrabAction>, InputMap<CrabAction>(player_input_map), Crab)]
pub struct Player;

fn player_input_map() -> InputMap<CrabAction> {
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
    ])
    .with_multiple([
        (MoveUp, GamepadButton::DPadUp),
        (MoveDown, GamepadButton::DPadDown),
        (MoveLeft, GamepadButton::DPadLeft),
        (MoveRight, GamepadButton::DPadRight),
    ]);
    // // TODO: Figure out why gamepad bindings keeps causing a panic!
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

    input_map
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
