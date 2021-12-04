use bevy::{ecs::prelude::*, prelude::*};

use crate::GameConfig;

#[derive(Component)]
pub struct Active;

#[derive(Clone, Component, Copy, PartialEq, Debug)]
pub enum Fade {
    Out(f32),
    In(f32),
}

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
    mut query: Query<(Entity, &mut Fade)>,
) {
    // Simulates fade from visible->invisible and vice versa over time
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
