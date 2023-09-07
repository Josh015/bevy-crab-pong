use crate::prelude::*;

/// A component for marking a [`Paddle`] entity as being controller by player
/// input.
#[derive(Component)]
pub struct KeyboardControlled;

/// Handles all user input regardless of the current game state.
fn keyboard_controlled_paddles(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    query: Query<Entity, (With<KeyboardControlled>, With<Paddle>)>,
) {
    // Makes a Paddle entity move left/right in response to the
    // keyboard's corresponding arrows keys.
    for entity in &query {
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

pub struct KeyboardPlugin;

impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            keyboard_controlled_paddles.in_set(GameSystemSet::GameplayLogic),
        );
    }
}
