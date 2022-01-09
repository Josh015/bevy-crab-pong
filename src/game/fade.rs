use super::*;
use crate::GameConfig;

/// A component that handles fading an entity in/out of visibility and marking
/// it as `Active`.
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

/// Immediately removes the `Active` component for entities that are `Fade::Out`
/// to ensure they aren't in play while disappearing.
pub fn begin_fade_system(
    mut commands: Commands,
    query: Query<(Entity, &Fade), Added<Fade>>,
) {
    for (entity, fade) in query.iter() {
        if matches!(*fade, Fade::Out(_)) {
            commands.entity(entity).remove::<Active>();
        }
    }
}

/// Handles the transition from visible->invisible and vice versa over time.
pub fn step_fade_system(
    mut commands: Commands,
    config: Res<GameConfig>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Fade)>,
) {
    for (entity, mut fade) in query.iter_mut() {
        let step = config.fade_speed * time.delta_seconds();

        match *fade {
            Fade::In(weight) => {
                if weight < 1.0 {
                    *fade = Fade::In(weight.max(0.0) + step);
                } else {
                    commands.entity(entity).remove::<Fade>().insert(Active);
                }
            },
            Fade::Out(weight) => {
                if weight < 1.0 {
                    *fade = Fade::Out(weight.max(0.0) + step);
                } else {
                    commands.entity(entity).remove::<Fade>();
                }
            },
        }
    }
}
