use crate::prelude::*;

// TODO: Go around and replace all GoalSide clone() calls since it is copyable?

pub const GOAL_WIDTH: f32 = 1.0;
pub const GOAL_HALF_WIDTH: f32 = 0.5 * GOAL_WIDTH;
pub const GOAL_PADDLE_START_POSITION: Vec3 = Vec3::new(0.0, 0.05, 0.0);
pub const GOAL_PADDLE_MAX_POSITION_X: f32 =
    GOAL_HALF_WIDTH - BARRIER_RADIUS - PADDLE_HALF_WIDTH;
pub const WALL_DIAMETER: f32 = 0.05;
pub const WALL_HEIGHT: f32 = 0.1;
pub const WALL_RADIUS: f32 = 0.5 * WALL_DIAMETER;
pub const WALL_SCALE: Vec3 =
    Vec3::new(GOAL_WIDTH, WALL_DIAMETER, WALL_DIAMETER);
pub const BALL_DIAMETER: f32 = 0.08;
pub const BALL_HEIGHT: f32 = 0.05;
pub const BALL_RADIUS: f32 = 0.5 * BALL_DIAMETER;
pub const BARRIER_RADIUS: f32 = 0.5 * BARRIER_DIAMETER;
pub const BARRIER_DIAMETER: f32 = 0.12;
pub const BARRIER_HEIGHT: f32 = 0.2;
pub const PADDLE_WIDTH: f32 = 0.2;
pub const PADDLE_DEPTH: f32 = 0.1;
pub const PADDLE_HALF_WIDTH: f32 = 0.5 * PADDLE_WIDTH;
pub const PADDLE_HALF_DEPTH: f32 = 0.5 * PADDLE_DEPTH;
pub const PADDLE_SCALE: Vec3 =
    Vec3::new(PADDLE_WIDTH, PADDLE_DEPTH, PADDLE_DEPTH);

/// An event fired when a [`Wall`] needs to be spawned.
pub struct SpawnWallEvent {
    pub side: Side,
    pub is_instant: bool,
}

/// An event fired when a [`Goal`] has been eliminated from play after its HP
/// has reached zero.
pub struct GoalEliminatedEvent(pub Side);

/// Marks a [`Goal`] entity so that [`Paddle`] and [`Wall`] entities can use it
/// as a parent, and so [`Ball`] entities can score against it.
#[derive(Component)]
pub struct Goal;

/// Spawns [`Paddle`] entities for their corresponding goals.
pub fn spawn_paddles_system(
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
                paddle.insert(Enemy);
            }
        });
    }
}

pub fn spawn_wall_event_system(
    run_state: Res<RunState>,
    mut commands: Commands,
    mut event_reader: EventReader<SpawnWallEvent>,
    goals_query: Query<(Entity, &Side), With<Goal>>,
) {
    for SpawnWallEvent { side, is_instant } in event_reader.iter() {
        for (entity, matching_side) in &goals_query {
            if *side != *matching_side {
                continue;
            }

            commands.entity(entity).with_children(|parent| {
                parent.spawn((
                    Wall,
                    side.clone(),
                    Collider,
                    FadeBundle {
                        fade_animation: FadeAnimation::Scale {
                            max_scale: WALL_SCALE,
                            axis_mask: Vec3::new(0.0, 1.0, 1.0),
                        },
                        fade: Fade::In(if *is_instant { 1.0 } else { 0.0 }),
                    },
                    PbrBundle {
                        mesh: run_state.wall_mesh_handle.clone(),
                        material: run_state.wall_material_handle.clone(),
                        transform: Transform::from_matrix(
                            Mat4::from_scale_rotation_translation(
                                Vec3::splat(f32::EPSILON),
                                Quat::IDENTITY,
                                Vec3::new(0.0, WALL_HEIGHT, 0.0),
                            ),
                        ),
                        ..default()
                    },
                ));
            });
            break;
        }
    }
}

/// Restricts a [`Paddle`] entity to the space between the [`Barrier`] entities
/// on either side of its [`Goal`].
pub fn goal_paddle_collision_system(
    mut query: Query<
        (&mut Transform, &mut Force, &mut Speed),
        (With<Paddle>, With<Collider>),
    >,
) {
    for (mut transform, mut force, mut speed) in &mut query {
        // Limit paddle to open space between barriers
        if !(-GOAL_PADDLE_MAX_POSITION_X..=GOAL_PADDLE_MAX_POSITION_X)
            .contains(&transform.translation.x)
        {
            transform.translation.x = transform
                .translation
                .x
                .clamp(-GOAL_PADDLE_MAX_POSITION_X, GOAL_PADDLE_MAX_POSITION_X);
            *force = Force::Zero;
            speed.0 = 0.0;
        }
    }
}

/// AI control for [`Paddle`] entities.
pub fn goal_paddle_ai_control_system(
    mut paddles_query: Query<
        (&Side, &Transform, &mut Force, &Speed, &Acceleration),
        (With<Paddle>, With<Enemy>),
    >,
    balls_query: Query<&GlobalTransform, (With<Ball>, With<Collider>)>,
) {
    // We want the paddle to follow and try to stay under the moving ball
    // rather than going straight to where it will cross the goal.
    for (side, transform, mut force, speed, acceleration) in &mut paddles_query
    {
        // Get the relative position of the ball that's closest to this goal.
        let mut closest_ball_distance = std::f32::MAX;
        let mut ball_local_position = GOAL_PADDLE_START_POSITION.x;

        for ball_transform in &balls_query {
            let ball_distance_to_goal = side.distance_to_ball(ball_transform);

            if ball_distance_to_goal >= closest_ball_distance {
                continue;
            }

            closest_ball_distance = ball_distance_to_goal;
            ball_local_position =
                side.map_ball_position_to_paddle_local_space(ball_transform);
        }

        // Predict the paddle's stop position if it begins decelerating now.
        let delta_seconds = 0.05; // Overshoots the ball slightly more often.
        // let delta_seconds = 0.001; // Precisely follows the ball.
        let delta_speed = acceleration.0 * delta_seconds;
        let mut current_speed = speed.0;
        let mut paddle_stop_position = transform.translation.x;

        while current_speed.abs() > 0.0 {
            paddle_stop_position += current_speed * delta_seconds;
            current_speed = decelerate_speed(current_speed, delta_speed);
        }

        // Controls how much the paddle tries to get its center under the ball.
        // Lower values improve the catch rate, but also reduce how widely it
        // will deflect the ball for near misses. Range (0,1].
        let percent_from_center = 0.60;
        let distance_from_paddle_center =
            (paddle_stop_position - ball_local_position).abs();

        *force = if distance_from_paddle_center
            < percent_from_center * PADDLE_HALF_WIDTH
        {
            Force::Zero
        } else if ball_local_position < transform.translation.x {
            Force::Negative // Left
        } else {
            Force::Positive // Right
        };
    }
}

/// Checks if a [`Ball`] has scored against a [`Goal`] and then decrements the
/// corresponding score.
pub fn goal_scored_check_system(
    mut commands: Commands,
    mut run_state: ResMut<RunState>,
    mut fade_out_entity_events: EventWriter<FadeOutEntityEvent>,
    mut goal_eliminated_writer: EventWriter<GoalEliminatedEvent>,
    balls_query: Query<
        (Entity, &GlobalTransform),
        (With<Ball>, With<Collider>),
    >,
    goals_query: Query<&Side, With<Goal>>,
) {
    for (entity, global_transform) in &balls_query {
        for side in &goals_query {
            // A ball will score against the goal it's closest to once it's
            // fully past the goal's paddle.
            let ball_distance = side.distance_to_ball(global_transform);

            if ball_distance > -PADDLE_HALF_DEPTH {
                continue;
            }

            // Decrement the goal's HP and potentially eliminate it.
            let hit_points = run_state.goals_hit_points.get_mut(side).unwrap();

            *hit_points = hit_points.saturating_sub(1);
            info!("Ball({:?}) -> Scored Goal({:?})", entity, side);

            if *hit_points == 0 {
                goal_eliminated_writer.send(GoalEliminatedEvent(*side));
                info!("Ball({:?}) -> Eliminated Goal({:?})", entity, side);
            }

            // Remove Collider and start fading out the ball to prevent
            // repeated scoring.
            commands.entity(entity).remove::<Collider>();
            fade_out_entity_events.send(FadeOutEntityEvent(entity));
            break;
        }
    }
}

/// Disables a given [`Goal`] to remove it from play.
pub fn goal_eliminated_event_system(
    mut commands: Commands,
    mut event_reader: EventReader<GoalEliminatedEvent>,
    mut fade_out_entity_events: EventWriter<FadeOutEntityEvent>,
    mut spawn_wall_events: EventWriter<SpawnWallEvent>,
    paddles_query: Query<
        (Entity, &Side),
        (With<Paddle>, With<Collider>, Without<Fade>),
    >,
) {
    for GoalEliminatedEvent(eliminated_side) in event_reader.iter() {
        // Fade out the paddle for the eliminated goal.
        for (entity, side) in &paddles_query {
            if *side != *eliminated_side {
                continue;
            }

            // Stop the paddle from moving and colliding.
            commands
                .entity(entity)
                .remove::<(Collider, Heading, Speed)>();
            fade_out_entity_events.send(FadeOutEntityEvent(entity));
            break;
        }

        // Fade in the wall for the eliminated goal.
        spawn_wall_events.send(SpawnWallEvent {
            side: eliminated_side.clone(),
            is_instant: false,
        });
    }
}

/// Fades out any existing [`Wall`] entities.
pub fn goal_despawn_walls_system(
    mut commands: Commands,
    mut fade_out_entity_events: EventWriter<FadeOutEntityEvent>,
    query: Query<Entity, With<Wall>>,
) {
    for entity in &query {
        commands.entity(entity).remove::<Collider>();
        fade_out_entity_events.send(FadeOutEntityEvent(entity));
    }
}
