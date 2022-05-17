use crate::prelude::*;

/// A component that handles fading an entity in/out of visibility and
/// despawning it if necessary.
///
/// The component-specific visual implementation of the fade in/out effect is
/// the responsibility of said component.
#[derive(Clone, Component, Copy, PartialEq, Debug)]
#[component(storage = "SparseSet")]
pub enum Fade {
    /// Simulates a fade-in effect, using a weight in the range \[0,1\].
    In(f32),

    /// Simulates a fade-out effect, using a weight in the range \[0,1\].
    Out(f32),
}

impl Fade {
    /// Returns the opacity of the current `Fade` type.
    pub fn opacity(&self) -> f32 {
        match *self {
            Self::In(weight) => weight,
            Self::Out(weight) => 1.0 - weight,
        }
    }
}

/// Handles the transition from visible->invisible and vice versa over time.
pub fn step_fade_system(
    mut commands: Commands,
    config: Res<GameConfig>,
    time: Res<Time>,
    state: Res<State<AppState>>,
    mut query: Query<(Entity, &mut Fade)>,
) {
    // Need to pause animations so balls launch correctly.
    if *state.current() == AppState::Pause {
        return;
    }

    for (entity, mut fade) in query.iter_mut() {
        let step = config.fade_speed * time.delta_seconds();

        match *fade {
            Fade::In(weight) => {
                if weight < 1.0 {
                    *fade = Fade::In(weight.max(0.0) + step);
                } else {
                    commands.entity(entity).remove::<Fade>();
                }
            },
            Fade::Out(weight) => {
                if weight < 1.0 {
                    *fade = Fade::Out(weight.max(0.0) + step);
                } else {
                    commands.entity(entity).despawn_recursive();
                }
            },
        }
    }
}
