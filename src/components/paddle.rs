use crate::prelude::*;

pub const PADDLE_WIDTH: f32 = 0.2;
pub const PADDLE_DEPTH: f32 = 0.1;
pub const PADDLE_HALF_WIDTH: f32 = 0.5 * PADDLE_WIDTH;
pub const PADDLE_HALF_DEPTH: f32 = 0.5 * PADDLE_DEPTH;
pub const PADDLE_SCALE: Vec3 =
    Vec3::new(PADDLE_WIDTH, PADDLE_DEPTH, PADDLE_DEPTH);

/// A component that makes a paddle that can deflect [`Ball`] entities and
/// moves left->right and vice versa along a single axis when [`Collider`].
#[derive(Clone, Component, Eq, PartialEq, Debug, Hash)]
pub struct Paddle;

/// Spawns [`Paddle`] entities for their corresponding goals.
pub fn spawn_paddles(
    mut commands: Commands,
    config: Res<GameConfig>,
    run_state: Res<RunState>,
    mut fade_out_entity_events: EventWriter<FadeOutEntityEvent>,
    paddles_query: Query<Entity, (With<Paddle>, Without<Fade>)>,
    goals_query: Query<(Entity, &Side), With<Goal>>,
) {
    // Fade out existing paddles so new ones can spawn at starting positions.
    for entity in &paddles_query {
        commands
            .entity(entity)
            .remove::<(Collider, Heading, Speed)>();
        fade_out_entity_events.send(FadeOutEntityEvent(entity));
    }

    // Give every paddle a parent so we can use relative transforms.
    for (i, (entity, side)) in goals_query.iter().enumerate() {
        commands.entity(entity).with_children(|parent| {
            let mut paddle = parent.spawn((
                Paddle,
                side.clone(),
                Collider,
                FadeBundle {
                    fade_animation: FadeAnimation::Scale {
                        max_scale: PADDLE_SCALE,
                        axis_mask: Vec3::ONE,
                    },
                    ..default()
                },
                AccelerationBundle {
                    velocity: VelocityBundle {
                        heading: Heading(Vec3::X),
                        ..default()
                    },
                    max_speed: MaxSpeed(config.paddle_max_speed),
                    acceleration: Acceleration(
                        config.paddle_max_speed
                            / config.paddle_seconds_to_max_speed,
                    ),
                    ..default()
                },
                PbrBundle {
                    mesh: run_state.paddle_mesh_handle.clone(),
                    material: run_state.paddle_material_handles[side].clone(),
                    transform: Transform::from_matrix(
                        Mat4::from_scale_rotation_translation(
                            Vec3::splat(f32::EPSILON),
                            Quat::IDENTITY,
                            GOAL_PADDLE_START_POSITION,
                        ),
                    ),
                    ..default()
                },
            ));

            // TODO: Combine with above statement after player selection
            // is fixed.
            if i == 0 {
                paddle.insert(Player);
            } else {
                paddle.insert(Ai);
            }
        });
    }
}

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_exit(AppState::StartMenu).with_system(spawn_paddles),
        );
    }
}
