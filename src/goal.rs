use crate::prelude::*;

// TODO: Go around and replace all GoalSide clone() calls since it is copyable?

pub const GOAL_WIDTH: f32 = 1.0;
pub const GOAL_HALF_WIDTH: f32 = 0.5 * GOAL_WIDTH;
pub const GOAL_PADDLE_START_POSITION: Vec3 = const_vec3!([0.0, 0.05, 0.0]);
pub const GOAL_PADDLE_MAX_POSITION_X: f32 =
    GOAL_HALF_WIDTH - BARRIER_RADIUS - PADDLE_HALF_WIDTH;
pub const WALL_DIAMETER: f32 = 0.05;
pub const WALL_HEIGHT: f32 = 0.1;
pub const WALL_RADIUS: f32 = 0.5 * WALL_DIAMETER;
pub const WALL_SCALE: Vec3 =
    const_vec3!([GOAL_WIDTH, WALL_DIAMETER, WALL_DIAMETER]);
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
    const_vec3!([PADDLE_WIDTH, PADDLE_DEPTH, PADDLE_DEPTH]);

/// An event fired when a `Wall` needs to be spawned.
pub struct SpawnWallEvent {
    pub side: Side,
    pub is_instant: bool,
}

/// An event fired when a `Ball` entity has scored inside a 'Goal'.
pub struct GoalScoredEvent {
    pub ball_entity: Entity,
}

/// An event fired when a `Goal` has been eliminated from play after its HP has
/// reached zero.
pub struct GoalEliminatedEvent(pub Side);

/// Marks a `Goal` entity so that `Paddle` and `Wall` entities can use it as a
/// parent, and so `Ball` entities can score against it.
#[derive(Component)]
pub struct Goal;

/// Spawns `Paddle` entities for their corresponding goals.
pub fn spawn_paddles_system(
    config: Res<GameConfig>,
    run_state: Res<RunState>,
    mut commands: Commands,
    paddles_query: Query<Entity, (With<Paddle>, Without<Fade>)>,
    goals_query: Query<(Entity, &Side), With<Goal>>,
) {
    // Fade out existing paddles so new ones can spawn at starting positions.
    for entity in paddles_query.iter() {
        fade_out_and_stop_entity(&mut commands, entity);
    }

    // Give every paddle a parent so we can use relative transforms.
    for (i, (entity, side)) in goals_query.iter().enumerate() {
        commands.entity(entity).with_children(|parent| {
            let mut paddle = parent.spawn_bundle(PbrBundle {
                mesh: run_state.paddle_mesh_handle.clone(),
                material: run_state.paddle_material_handles[side].clone(),
                transform: Transform::from_matrix(
                    Mat4::from_scale_rotation_translation(
                        Vec3::splat(f32::EPSILON),
                        Quat::IDENTITY,
                        GOAL_PADDLE_START_POSITION,
                    ),
                ),
                ..Default::default()
            });

            // TODO: Combine with above statement after player selection
            // is fixed.
            paddle.insert_bundle((
                side.clone(),
                Paddle,
                Collider,
                Movement {
                    direction: Vec3::X,
                    max_speed: config.paddle_max_speed,
                    acceleration: config.paddle_max_speed
                        / config.paddle_seconds_to_max_speed,
                    ..Default::default()
                },
                FadeAnimation::Scaling {
                    max_scale: PADDLE_SCALE,
                    axis_mask: Vec3::ONE,
                },
                Fade::In(0.0),
            ));

            // TODO: Come up with a more configurable way to do this!
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
        for (entity, matching_side) in goals_query.iter() {
            if *side != *matching_side {
                continue;
            }

            commands.entity(entity).with_children(|parent| {
                parent
                    .spawn_bundle(PbrBundle {
                        mesh: run_state.wall_mesh_handle.clone(),
                        material: run_state.wall_material_handle.clone(),
                        transform: Transform::from_matrix(
                            Mat4::from_scale_rotation_translation(
                                Vec3::splat(f32::EPSILON),
                                Quat::IDENTITY,
                                Vec3::new(0.0, WALL_HEIGHT, 0.0),
                            ),
                        ),
                        ..Default::default()
                    })
                    .insert_bundle((
                        side.clone(),
                        Wall,
                        Collider,
                        FadeAnimation::Scaling {
                            max_scale: WALL_SCALE,
                            axis_mask: Vec3::new(0.0, 1.0, 1.0),
                        },
                        Fade::In(if *is_instant { 1.0 } else { 0.0 }),
                    ));
            });
            break;
        }
    }
}

/// Restricts a `Paddle` entity to the space between the `Barrier` entities on
/// either side of its `Goal`.
pub fn goal_paddle_collision_system(
    mut query: Query<
        (&mut Transform, &mut Movement),
        (With<Paddle>, With<Collider>),
    >,
) {
    for (mut transform, mut movement) in query.iter_mut() {
        // Limit paddle to open space between barriers
        if transform.translation.x > GOAL_PADDLE_MAX_POSITION_X {
            transform.translation.x = GOAL_PADDLE_MAX_POSITION_X;
            movement.stop();
        } else if transform.translation.x < -GOAL_PADDLE_MAX_POSITION_X {
            transform.translation.x = -GOAL_PADDLE_MAX_POSITION_X;
            movement.stop();
        }
    }
}

/// AI control for `Paddle` entities.
pub fn goal_paddle_ai_control_system(
    time: Res<Time>,
    mut paddles_query: Query<
        (&Side, &Transform, &mut Movement),
        (With<Paddle>, With<Enemy>),
    >,
    balls_query: Query<&GlobalTransform, (With<Ball>, With<Collider>)>,
) {
    for (side, transform, mut movement) in paddles_query.iter_mut() {
        // Get the relative position of the ball that's closest to this goal.
        let mut closest_ball_distance = std::f32::MAX;
        let mut ball_local_position = GOAL_PADDLE_START_POSITION.x;

        for ball_transform in balls_query.iter() {
            let ball_distance_to_goal = side.distance_to_ball(ball_transform);

            if ball_distance_to_goal >= closest_ball_distance {
                continue;
            }

            closest_ball_distance = ball_distance_to_goal;
            ball_local_position =
                side.map_ball_position_to_paddle_local_space(ball_transform);
        }

        // Predict the paddle's stop position if it begins decelerating now.
        let delta_seconds = time.delta_seconds();
        let delta_speed = movement.acceleration * delta_seconds;
        let mut current_speed = movement.speed;
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

        movement.delta = if distance_from_paddle_center
            < percent_from_center * PADDLE_HALF_WIDTH
        {
            None
        } else if ball_local_position < transform.translation.x {
            Some(MovementDelta::Negative) // Left
        } else {
            Some(MovementDelta::Positive) // Right
        };
    }
}

/// Checks if a `Ball` has scored against a `Goal` and then decrements the
/// corresponding score.
pub fn goal_scored_check_system(
    mut run_state: ResMut<RunState>,
    mut goal_scored_writer: EventWriter<GoalScoredEvent>,
    mut goal_eliminated_writer: EventWriter<GoalEliminatedEvent>,
    balls_query: Query<
        (Entity, &GlobalTransform),
        (With<Ball>, With<Collider>),
    >,
) {
    // TODO: Make this shareable.
    let sides = [Side::Top, Side::Right, Side::Bottom, Side::Left];

    for (ball_entity, global_transform) in balls_query.iter() {
        for side in sides {
            // A ball will score against the goal it's closest to once it's
            // fully past the goal's paddle.
            let ball_distance = side.distance_to_ball(global_transform);

            if ball_distance > -PADDLE_HALF_DEPTH {
                continue;
            }

            // Decrement the goal's HP and potentially eliminate it.
            let hit_points = run_state.goals_hit_points.get_mut(&side).unwrap();

            *hit_points = hit_points.saturating_sub(1);
            goal_scored_writer.send(GoalScoredEvent { ball_entity });
            info!("Ball({:?}) -> Scored Goal({:?})", ball_entity, side);

            if *hit_points == 0 {
                goal_eliminated_writer.send(GoalEliminatedEvent(side));
                info!("Ball({:?}) -> Eliminated Goal({:?})", ball_entity, side);
            }
            break;
        }
    }
}

/// Fades out a `Ball` entity when it scores in a 'Goal' and prevents it from
/// repeatedly scoring/colliding as it fades.
pub fn goal_scored_event_system(
    mut commands: Commands,
    mut event_reader: EventReader<GoalScoredEvent>,
) {
    for GoalScoredEvent { ball_entity } in event_reader.iter() {
        fade_out_entity(&mut commands, *ball_entity);
    }
}

/// Disables a given `Goal` to remove it from play.
pub fn goal_eliminated_event_system(
    mut commands: Commands,
    mut event_reader: EventReader<GoalEliminatedEvent>,
    mut spawn_wall_events: EventWriter<SpawnWallEvent>,
    paddles_query: Query<
        (Entity, &Side),
        (With<Paddle>, With<Collider>, Without<Fade>),
    >,
) {
    for GoalEliminatedEvent(eliminated_side) in event_reader.iter() {
        // Fade out the paddle for the eliminated goal.
        for (entity, side) in paddles_query.iter() {
            if *side != *eliminated_side {
                continue;
            }

            // Stop the paddle from moving and colliding.
            fade_out_and_stop_entity(&mut commands, entity);
            break;
        }

        // Fade in the wall for the eliminated goal.
        spawn_wall_events.send(SpawnWallEvent {
            side: eliminated_side.clone(),
            is_instant: false,
        });
    }
}

/// Fades out any existing `Wall` entities.
pub fn goal_despawn_walls_system(
    mut commands: Commands,
    query: Query<Entity, (With<Wall>, Without<Fade>)>,
) {
    for entity in query.iter() {
        fade_out_entity(&mut commands, entity);
    }
}
