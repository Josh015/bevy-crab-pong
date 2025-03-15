use bevy::prelude::*;

use crate::{
    components::{
        Collider, Fade, FadeEffect, POLE_DIAMETER, POLE_HEIGHT, Pole,
        RemoveBeforeFadeOut,
    },
    game::GOAL_WIDTH,
};

use super::CachedAssets;

pub(super) struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_pole_in_a_goal)
            .add_event::<GoalEliminatedEvent>()
            .add_event::<GoalScoredEvent>();
    }
}

#[derive(Event)]
pub struct SpawnPole {
    pub goal_entity: Entity,
    pub fade_in: bool,
}

/// Signal when a goal entity has been scored by a ball.
#[derive(Clone, Debug, Event)]
pub struct GoalScoredEvent(pub Entity);

/// Signals that a goal has been eliminated from the game.
#[derive(Clone, Debug, Event)]
pub struct GoalEliminatedEvent(pub Entity);

fn spawn_pole_in_a_goal(
    trigger: Trigger<SpawnPole>,
    cached_assets: Res<CachedAssets>,
    mut commands: Commands,
) {
    let SpawnPole {
        goal_entity,
        fade_in,
    } = trigger.event();

    let id = commands
        .entity(*goal_entity)
        .with_children(|builder| {
            builder.spawn((
                Pole,
                Collider,
                RemoveBeforeFadeOut::<Collider>::default(),
                if *fade_in {
                    Fade::new_in()
                } else {
                    Fade::In(Timer::default()) // Skip to end of animation.
                },
                FadeEffect::Scale {
                    max_scale: Vec3::new(
                        POLE_DIAMETER,
                        GOAL_WIDTH,
                        POLE_DIAMETER,
                    ),
                    axis_mask: Vec3::new(1.0, 0.0, 1.0),
                },
                Mesh3d(cached_assets.pole_mesh.clone()),
                MeshMaterial3d(cached_assets.pole_material.clone()),
                Transform::from_matrix(Mat4::from_scale_rotation_translation(
                    Vec3::splat(f32::EPSILON),
                    Quat::from_euler(
                        EulerRot::XYZ,
                        0.0,
                        0.0,
                        std::f32::consts::FRAC_PI_2,
                    ),
                    Vec3::new(0.0, POLE_HEIGHT, 0.0),
                )),
            ));
        })
        .id();

    info!("Pole({id:?}): Spawned");
}
