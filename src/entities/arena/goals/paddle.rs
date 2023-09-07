#![allow(clippy::type_complexity)]

use crate::prelude::*;
use std::collections::HashMap;

pub const PADDLE_WIDTH: f32 = 0.2;
pub const PADDLE_DEPTH: f32 = 0.1;
pub const PADDLE_HALF_WIDTH: f32 = 0.5 * PADDLE_WIDTH;
pub const PADDLE_HALF_DEPTH: f32 = 0.5 * PADDLE_DEPTH;
pub const PADDLE_SCALE: Vec3 =
    Vec3::new(PADDLE_WIDTH, PADDLE_DEPTH, PADDLE_DEPTH);
pub const PADDLE_CENTER_HIT_AREA_PERCENTAGE: f32 = 0.5;

/// A component that makes a paddle that can deflect [`Ball`] entities and
/// moves left->right and vice versa along a single axis when [`Collider`].
#[derive(Clone, Component, Eq, PartialEq, Debug, Hash)]
pub struct Paddle;

/// The ball being targeted by AI paddles.
#[derive(Component)]
pub struct Target(pub Entity);

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
                (Side::Top, materials.add(Color::ORANGE.into())),
                (Side::Right, materials.add(Color::BLUE.into())),
                (Side::Bottom, materials.add(Color::RED.into())),
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
    game_state: Res<GameState>,
    game_config: Res<GameConfig>,
    resources: Res<PaddleResources>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
        let goal_config = &game_config.modes[game_state.mode_index].goals[i];
        let material_handle = resources.paddle_material_handles[side].clone();

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
                    max_speed: MaxSpeed(game_config.paddle_max_speed),
                    acceleration: Acceleration(
                        game_config.paddle_max_speed
                            / game_config.paddle_seconds_to_max_speed,
                    ),
                    ..default()
                },
                PbrBundle {
                    mesh: resources.paddle_mesh_handle.clone(),
                    material: material_handle.clone(),
                    transform: Transform::from_matrix(
                        Mat4::from_scale_rotation_translation(
                            Vec3::splat(f32::EPSILON),
                            Quat::IDENTITY,
                            GOAL_PADDLE_START_POSITION,
                        ),
                    ),
                    ..default()
                },
                if goal_config.team == TeamConfig::Enemies {
                    Team::Enemies
                } else {
                    Team::Allies
                },
            ));

            if goal_config.controlled_by == ControlledByConfig::AI {
                paddle.insert(AiInput);
            } else {
                paddle.insert(KeyboardInput);
            }

            let material = materials.get_mut(&material_handle).unwrap();
            material.base_color = Color::hex(&goal_config.color).unwrap()
        });
    }
}

/// Handles all user input regardless of the current game state.
fn keyboard_controlled_paddles(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    query: Query<Entity, (With<KeyboardInput>, With<Paddle>)>,
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

/// AI control for [`Paddle`] entities.
fn ai_controlled_paddles(
    mut commands: Commands,
    paddles_query: Query<
        (
            Entity,
            &Side,
            &Transform,
            &StoppingDistance,
            Option<&Target>,
        ),
        (With<AiInput>, With<Paddle>),
    >,
    balls_query: Query<&GlobalTransform, (With<Ball>, With<Collider>)>,
) {
    for (entity, side, transform, stopping_distance, target) in &paddles_query {
        // Use the ball's goal position or default to the center of the goal.
        let mut target_goal_position = FIELD_CENTER_POINT.x;

        if let Some(target) = target {
            if let Ok(ball_transform) = balls_query.get(target.0) {
                target_goal_position = side.get_ball_position(ball_transform)
            }
        }

        // Move the paddle so that its center is over the target goal position.
        let paddle_stop_position =
            transform.translation.x + stopping_distance.0;
        let distance_from_paddle_center =
            (paddle_stop_position - target_goal_position).abs();

        if distance_from_paddle_center
            < PADDLE_CENTER_HIT_AREA_PERCENTAGE * PADDLE_HALF_WIDTH
        {
            commands.entity(entity).remove::<Force>();
        } else {
            commands.entity(entity).insert(
                if target_goal_position < transform.translation.x {
                    Force::Negative // Left
                } else {
                    Force::Positive // Right
                },
            );
        }
    }
}

/// Causes [`Ai`] entities to target whichever ball is closest to their goal.
fn detect_and_target_ball_closest_to_goal(
    mut commands: Commands,
    paddles_query: Query<(Entity, &Side), (With<AiInput>, With<Paddle>)>,
    balls_query: Query<
        (Entity, &GlobalTransform),
        (With<Ball>, With<Collider>),
    >,
) {
    for (ai_entity, side) in &paddles_query {
        let mut closest_ball_distance = std::f32::MAX;
        let mut target = None;

        for (ball_entity, ball_transform) in &balls_query {
            let ball_distance_to_goal = side.distance_to_ball(ball_transform);

            if ball_distance_to_goal < closest_ball_distance {
                closest_ball_distance = ball_distance_to_goal;
                target = Some(ball_entity);
            }
        }

        if let Some(target) = target {
            commands.entity(ai_entity).insert(Target(target));
        } else {
            commands.entity(ai_entity).remove::<Target>();
        }
    }
}

/// Restricts a [`Paddle`] entity to the open space of its goal.
fn restrict_paddle_to_goal_space(
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut Transform, &mut Speed, &mut StoppingDistance),
        (With<Paddle>, With<Collider>),
    >,
) {
    for (entity, mut transform, mut speed, mut stopping_distance) in &mut query
    {
        // Limit paddle to bounds of the goal.
        if !GOAL_PADDLE_MAX_POSITION_RANGE.contains(&transform.translation.x) {
            transform.translation.x = transform
                .translation
                .x
                .clamp(-GOAL_PADDLE_MAX_POSITION_X, GOAL_PADDLE_MAX_POSITION_X);
            speed.0 = 0.0;
            commands.entity(entity).remove::<Force>();
        }

        // Limit stopping distance to the bounds of the goal.
        let stopped_position = transform.translation.x + stopping_distance.0;

        if !GOAL_PADDLE_MAX_POSITION_RANGE.contains(&stopped_position) {
            stopping_distance.0 = stopped_position.signum()
                * GOAL_PADDLE_MAX_POSITION_X
                - transform.translation.x;
        }
    }
}

/// Checks if a [`Ball`] and a [`Paddle`] have collided.
fn ball_to_paddle_collisions(
    mut commands: Commands,
    balls_query: Query<
        (Entity, &GlobalTransform, &Heading),
        (With<Ball>, With<Collider>),
    >,
    paddles_query: Query<(&Side, &Transform), (With<Paddle>, With<Collider>)>,
) {
    for (entity, ball_transform, ball_heading) in &balls_query {
        for (side, transform) in &paddles_query {
            let goal_axis = side.axis();
            let ball_distance_to_goal = side.distance_to_ball(ball_transform);
            let ball_goal_position = side.get_ball_position(ball_transform);
            let ball_to_paddle = transform.translation.x - ball_goal_position;
            let ball_distance_to_paddle = ball_to_paddle.abs();

            // Check that the ball is touching the paddle and facing the goal.
            if ball_distance_to_goal > PADDLE_HALF_DEPTH
                || ball_distance_to_paddle >= PADDLE_HALF_WIDTH
                || ball_heading.0.dot(goal_axis) <= 0.0
            {
                continue;
            }

            // Reverse the ball's direction and rotate it outward based on how
            // far its position is from the paddle's center.
            let rotation_away_from_center = Quat::from_rotation_y(
                std::f32::consts::FRAC_PI_4
                    * (ball_to_paddle / PADDLE_HALF_WIDTH),
            );
            commands
                .entity(entity)
                .insert(Heading(rotation_away_from_center * -ball_heading.0));

            info!("Ball({:?}): Collided Paddle({:?})", entity, side);
            break;
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

/// Provides debug visualization to show which [`Ai`] entities are targeting
/// which [`Ball`] entities.
fn debug_targeting(
    paddles_query: Query<
        (&GlobalTransform, &Target),
        (With<AiInput>, With<Paddle>, Without<Fade>),
    >,
    balls_query: Query<&GlobalTransform, (With<Ball>, With<Collider>)>,
    mut gizmos: Gizmos,
) {
    for (paddle_transform, target) in &paddles_query {
        if let Ok(ball_transform) = balls_query.get(target.0) {
            gizmos.line(
                paddle_transform.translation(),
                ball_transform.translation(),
                Color::PURPLE,
            );
        }
    }
}

/// Provides debug visualization to show the size of the ideal hit area on each
/// [`Ai`] [`Paddle`] entity.
fn debug_ai_paddle_hit_area(
    paddles_query: Query<
        &GlobalTransform,
        (With<Paddle>, With<AiInput>, Without<Fade>),
    >,
    mut gizmos: Gizmos,
) {
    for global_transform in &paddles_query {
        let mut hit_area_transform = global_transform.compute_transform();

        hit_area_transform.scale.x =
            PADDLE_CENTER_HIT_AREA_PERCENTAGE * PADDLE_WIDTH;
        gizmos.cuboid(hit_area_transform, Color::YELLOW);
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
                    (
                        keyboard_controlled_paddles,
                        detect_and_target_ball_closest_to_goal,
                        ai_controlled_paddles,
                    )
                        .chain()
                        .in_set(GameSystemSet::GameplayLogic),
                    restrict_paddle_to_goal_space
                        .in_set(GameSystemSet::Collision),
                    (
                        debug_paddle_stop_positions,
                        debug_targeting,
                        debug_ai_paddle_hit_area,
                    )
                        .in_set(GameSystemSet::Debugging),
                ),
            )
            .add_systems(
                PostUpdate,
                ball_to_paddle_collisions.in_set(GameSystemSet::Collision),
            );
    }
}
