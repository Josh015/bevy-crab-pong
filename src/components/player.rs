use crate::prelude::*;

/// A component for marking a [`Paddle`] entity as being controller by player
/// input.
#[derive(Component)]
pub struct Player;

/// Handles all user input regardless of the current game state.
fn player_paddle_control(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Force, (With<Player>, With<Paddle>)>,
) {
    // Makes a Paddle entity move left/right in response to the
    // keyboard's corresponding arrows keys.
    for mut force in &mut query {
        *force = if keyboard_input.pressed(KeyCode::Left)
            || keyboard_input.pressed(KeyCode::A)
        {
            Force::Negative
        } else if keyboard_input.pressed(KeyCode::Right)
            || keyboard_input.pressed(KeyCode::D)
        {
            Force::Positive
        } else {
            Force::Zero
        };
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            player_paddle_control.in_set(LogicalSet::GameplayLogic),
        );
    }
}
