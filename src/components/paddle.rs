use crate::prelude::*;
use std::collections::HashMap;

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

/// Cached paddle materials and meshes.
#[derive(Debug, Resource)]
pub struct PaddleResources {
    pub paddle_mesh_handle: Handle<Mesh>,
    pub paddle_material_handles: HashMap<Side, Handle<StandardMaterial>>,
}

impl FromWorld for PaddleResources {
    fn from_world(world: &mut World) -> Self {
        let paddle_mesh_handle = {
            let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();

            meshes.add(Mesh::from(shape::Cube { size: 1.0 }))
        };
        let paddle_material_handles = {
            let mut materials = world
                .get_resource_mut::<Assets<StandardMaterial>>()
                .unwrap();

            HashMap::from([
                (Side::Bottom, materials.add(Color::RED.into())),
                (Side::Right, materials.add(Color::BLUE.into())),
                (Side::Top, materials.add(Color::ORANGE.into())),
                (Side::Left, materials.add(Color::PURPLE.into())),
            ])
        };

        Self {
            paddle_mesh_handle,
            paddle_material_handles,
        }
    }
}

/// Spawns [`Paddle`] entities for their corresponding goals.
fn spawn_paddles(
    mut commands: Commands,
    config: Res<GameConfig>,
    resources: Res<PaddleResources>,
    mut fade_out_entity_events: EventWriter<FadeOutEntityEvent>,
    paddles_query: Query<Entity, (With<Paddle>, Without<Fade>)>,
    goals_query: Query<(Entity, &Side), With<Goal>>,
) {
    // Fade out existing paddles so new ones can spawn at starting positions.
    for entity in &paddles_query {
        commands
            .entity(entity)
            .remove::<(Collider, VelocityBundle)>();
        fade_out_entity_events.send(FadeOutEntityEvent(entity));
    }

    // Give every paddle a parent so we can use relative transforms.
    for (i, (entity, side)) in goals_query.iter().enumerate() {
        commands.entity(entity).with_children(|parent| {
            let mut paddle = parent.spawn((
                *side,
                Paddle,
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
                    mesh: resources.paddle_mesh_handle.clone(),
                    material: resources.paddle_material_handles[side].clone(),
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

/// Bounds check and restrict each paddles' stopping distance.
fn bounds_check_paddle_stopping_distance(
    mut query: Query<
        (&Transform, &mut StoppingDistance),
        (With<Paddle>, Without<Fade>),
    >,
) {
    for (transform, mut stopping_distance) in &mut query {
        let new_position = transform.translation.x + stopping_distance.0;

        if !GOAL_PADDLE_MAX_POSITION_RANGE.contains(&new_position) {
            stopping_distance.0 = new_position.signum()
                * GOAL_PADDLE_MAX_POSITION_X
                - transform.translation.x;
        }
    }
}

/// Visualizes where the paddles will be when they stop.
fn debug_paddle_stop_positions(
    query: Query<
        (&GlobalTransform, &Heading, &StoppingDistance),
        Without<Fade>,
    >,
    mut gizmos: Gizmos,
) {
    for (global_transform, heading, stopping_distance) in &query {
        let mut stop_position_transform = global_transform.compute_transform();
        let global_heading = stop_position_transform.rotation * heading.0;

        stop_position_transform.translation +=
            global_heading * stopping_distance.0;
        gizmos.line(
            global_transform.translation(),
            stop_position_transform.translation,
            Color::BLUE,
        );
        gizmos.cuboid(stop_position_transform, Color::GREEN);
    }
}

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PaddleResources>()
            .add_systems(OnExit(GameScreen::StartMenu), spawn_paddles)
            .add_systems(
                Update,
                (
                    bounds_check_paddle_stopping_distance
                        .in_set(GameSystemSet::Collision),
                    debug_paddle_stop_positions
                        .in_set(GameSystemSet::Debugging),
                ),
            );
    }
}
