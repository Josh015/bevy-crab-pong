use bevy::{ecs::prelude::*, prelude::*};

use crate::GameConfig;

#[derive(Clone, Component, Copy, PartialEq, Debug)]
pub enum Fade {
    In(f32),
    Out(f32),
}

#[derive(Component)]
pub struct Active;

impl Fade {
    pub fn opacity(&self) -> f32 {
        match self {
            Self::In(weight) => *weight,
            Self::Out(weight) => 1.0 - weight,
        }
    }
}

pub fn start_fade_system(
    mut commands: Commands,
    query: Query<(Entity, &Fade), Added<Fade>>,
) {
    for (entity, fade) in query.iter() {
        if matches!(*fade, Fade::Out(_)) {
            commands.entity(entity).remove::<Active>();
        }
    }
}

pub fn step_fade_system(
    mut commands: Commands,
    config: Res<GameConfig>,
    time: Res<Time>,
    query: Query<(Entity, &Fade)>,
) {
    // Simulates fade from visible->invisible and vice versa over time
    for (entity, fade) in query.iter() {
        let mut entity_commands = commands.entity(entity);
        let step = config.fade_speed * time.delta_seconds();

        match *fade {
            Fade::In(weight) => {
                if weight < 1.0 {
                    entity_commands.insert(Fade::In(weight.max(0.0) + step));
                } else {
                    entity_commands.remove::<Fade>().insert(Active);
                }
            },
            Fade::Out(weight) => {
                if weight < 1.0 {
                    entity_commands.insert(Fade::Out(weight.max(0.0) + step));
                } else {
                    entity_commands.remove::<Fade>();
                }
            },
        }
    }
}
