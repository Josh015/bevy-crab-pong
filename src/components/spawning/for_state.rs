use crate::prelude::*;

/// Tags an entity to only exist in the listed game states.
#[derive(Component)]
pub struct ForState<T> {
    pub states: Vec<T>,
}

/// Check [`ForState`] entities and either fade out or despawn any that aren't
/// allowed in the current [`AppState`].
fn despawn_invalid_entities_for_state(
    mut commands: Commands,
    game_screen: Res<State<GameScreen>>,
    mut fade_out_entity_events: EventWriter<FadeOutEntityEvent>,
    mut query: Query<(Entity, &ForState<GameScreen>, Option<&FadeAnimation>)>,
) {
    for (entity, for_state, fade_animation) in &mut query {
        if for_state.states.contains(game_screen.get()) {
            continue;
        }

        if fade_animation.is_some() {
            fade_out_entity_events.send(FadeOutEntityEvent(entity));
        } else {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct ForStatePlugin;

impl Plugin for ForStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RunState>().add_systems(
            PostUpdate,
            despawn_invalid_entities_for_state
                .run_if(state_changed::<GameScreen>()),
        );
    }
}
