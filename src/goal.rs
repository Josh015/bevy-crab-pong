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
    pub goal_side: GoalSide,
    pub is_instant: bool,
}

/// An event fired when a `Ball` entity has scored inside a 'Goal'.
pub struct GoalScoredEvent {
    pub ball_entity: Entity,
}

/// An event fired when a `Goal` has been eliminated from play after its HP has
/// reached zero.
pub struct GoalEliminatedEvent(pub GoalSide);

#[derive(Component)]
pub struct Goal {
    pub side: GoalSide,
}

/// Marks a `Goal` entity so that `Paddle` and `Wall` entities can use it as a
/// parent, and so `Ball` entities can score against it.
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum GoalSide {
    Top,
    Right,
    Bottom,
    Left,
}

impl GoalSide {
    /// Perpendicular distance from a given goal to a ball's edge.
    ///
    /// Positive distances for inside the arena, negative for out of bounds.
    pub fn distance_to_ball(&self, ball_transform: &GlobalTransform) -> f32 {
        let ball_translation = ball_transform.translation;

        match *self {
            Self::Top => GOAL_HALF_WIDTH + ball_translation.z - BALL_RADIUS,
            Self::Right => GOAL_HALF_WIDTH - ball_translation.x - BALL_RADIUS,
            Self::Bottom => GOAL_HALF_WIDTH - ball_translation.z - BALL_RADIUS,
            Self::Left => GOAL_HALF_WIDTH + ball_translation.x - BALL_RADIUS,
        }
    }

    /// Get the (+/-)(X/Z) axis the goal occupies.
    pub fn axis(&self) -> Vec3 {
        match *self {
            Self::Top => -Vec3::Z,
            Self::Right => Vec3::X,
            Self::Bottom => Vec3::Z,
            Self::Left => -Vec3::X,
        }
    }

    /// Map a ball's global position to a paddle's local x-axis.
    pub fn map_ball_position_to_paddle_range(
        &self,
        ball_transform: &GlobalTransform,
    ) -> f32 {
        match *self {
            Self::Top => -ball_transform.translation.x,
            Self::Right => -ball_transform.translation.z,
            Self::Bottom => ball_transform.translation.x,
            Self::Left => ball_transform.translation.z,
        }
    }
}

/// Spawns `Paddle` entities for their corresponding goals.
pub fn spawn_paddles(
    config: Res<GameConfig>,
    run_state: Res<RunState>,
    mut commands: Commands,
    mut paddles_query: Query<(Entity, &mut Fade), With<Paddle>>,
    goals_query: Query<(Entity, &Goal)>,
) {
    let goal_configs = [
        GoalSide::Bottom,
        GoalSide::Right,
        GoalSide::Top,
        GoalSide::Left,
    ];

    // Fade out existing paddles so new ones can spawn at starting positions.
    for (entity, mut fade) in paddles_query.iter_mut() {
        commands
            .entity(entity)
            .remove::<Movement>()
            .remove::<Collider>();
        fade.fade_out_and_despawn();
    }

    // TODO: Figure out why using goals_query alone only hits first item?
    // Does the iterator become invalid once we add children?

    // Give every paddle a parent so we can use relative transforms.
    for (i, paddle_goal) in goal_configs.iter().enumerate() {
        for (entity, goal) in goals_query.iter() {
            if goal.side != *paddle_goal {
                continue;
            }

            commands.entity(entity).with_children(|parent| {
                let mut paddle = parent.spawn_bundle(PbrBundle {
                    mesh: run_state.paddle_mesh_handle.clone(),
                    material: run_state.paddle_material_handles[paddle_goal]
                        .clone(),
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
                    Paddle {
                        goal_side: paddle_goal.clone(),
                    },
                    Collider,
                    Movement {
                        direction: Vec3::X,
                        max_speed: config.paddle_max_speed,
                        acceleration: config.paddle_max_speed
                            / config.paddle_seconds_to_max_speed,
                        ..Default::default()
                    },
                    Fade::new(FadeEffect::Scale {
                        max_scale: PADDLE_SCALE,
                        axis_mask: Vec3::ONE,
                    }),
                ));

                // TODO: Come up with a more configurable way to do this!
                if i == 0 {
                    paddle.insert(Player);
                } else {
                    paddle.insert(Enemy);
                }
            });
            break;
        }
    }
}

pub fn spawn_wall_event(
    run_state: Res<RunState>,
    mut commands: Commands,
    mut event_reader: EventReader<SpawnWallEvent>,
    goals_query: Query<(Entity, &Goal)>,
) {
    for SpawnWallEvent {
        goal_side,
        is_instant,
    } in event_reader.iter()
    {
        for (entity, matching_goal) in goals_query.iter() {
            if *goal_side != matching_goal.side {
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
                        Wall {
                            goal_side: goal_side.clone(),
                        },
                        Collider,
                        Fade::new_with_state(
                            FadeEffect::Scale {
                                max_scale: WALL_SCALE,
                                axis_mask: Vec3::new(0.0, 1.0, 1.0),
                            },
                            Some(FadeState::In(if *is_instant {
                                1.0
                            } else {
                                0.0
                            })),
                        ),
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

/// Applies AI control to `Paddle` entities, causing them to position
/// themselves between their `Goal` and the closest `Ball`.
pub fn goal_paddle_ai_control_system(
    mut paddles_query: Query<(&Paddle, &Transform, &mut Movement), With<Enemy>>,
    balls_query: Query<&GlobalTransform, (With<Ball>, With<Collider>)>,
) {
    for (paddle, transform, mut movement) in paddles_query.iter_mut() {
        // Get the relative position of the ball that's closest to this goal.
        let goal_side = paddle.goal_side;
        let mut closest_ball_distance = std::f32::MAX;
        let mut target_position = GOAL_PADDLE_START_POSITION.x;

        for ball_transform in balls_query.iter() {
            let ball_distance = goal_side.distance_to_ball(ball_transform);

            if ball_distance >= closest_ball_distance {
                continue;
            }

            closest_ball_distance = ball_distance;
            target_position =
                goal_side.map_ball_position_to_paddle_range(ball_transform);
        }

        // Predict the paddle's stop position if it begins decelerating now.
        let d = movement.speed.powi(2) / movement.acceleration;
        let stop_position = if movement.speed > 0.0 {
            transform.translation.x + d
        } else {
            transform.translation.x - d
        };

        // Position the paddle so that the ball is above its middle 70%.
        let distance_from_paddle_center =
            (stop_position - target_position).abs();

        movement.delta =
            if distance_from_paddle_center < 0.7 * PADDLE_HALF_WIDTH {
                None
            } else if target_position < transform.translation.x {
                Some(Delta::Negative) // Left
            } else {
                Some(Delta::Positive) // Right
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
    let goal_sides = [
        GoalSide::Top,
        GoalSide::Right,
        GoalSide::Bottom,
        GoalSide::Left,
    ];

    for (ball_entity, global_transform) in balls_query.iter() {
        for goal_side in goal_sides {
            // A ball will score against the goal it's closest to once it's
            // fully past the goal's paddle.
            let ball_distance = goal_side.distance_to_ball(global_transform);

            if ball_distance > -PADDLE_HALF_DEPTH {
                continue;
            }

            // Decrement the goal's HP and potentially eliminate it.
            let hit_points =
                run_state.goals_hit_points.get_mut(&goal_side).unwrap();

            *hit_points = hit_points.saturating_sub(1);
            goal_scored_writer.send(GoalScoredEvent { ball_entity });
            info!("Ball({:?}) -> Scored Goal({:?})", ball_entity, goal_side);

            if *hit_points == 0 {
                goal_eliminated_writer.send(GoalEliminatedEvent(goal_side));
                info!(
                    "Ball({:?}) -> Eliminated Goal({:?})",
                    ball_entity, goal_side
                );
            }
            break;
        }
    }
}

/// Fades out a `Ball` entity when it scores in a 'Goal' and prevents it from
/// repeatedly scoring/colliding as it fades.
pub fn goal_scored_event(
    mut commands: Commands,
    mut event_reader: EventReader<GoalScoredEvent>,
    mut query: Query<&mut Fade, With<Ball>>,
) {
    for GoalScoredEvent { ball_entity } in event_reader.iter() {
        commands.entity(*ball_entity).remove::<Collider>();

        if let Ok(mut fade) = query.get_component_mut::<Fade>(*ball_entity) {
            fade.fade_out_and_despawn();
        }
    }
}

/// Disables a given `Goal` to remove it from play.
pub fn goal_eliminated_event(
    mut commands: Commands,
    mut event_reader: EventReader<GoalEliminatedEvent>,
    mut spawn_wall_events: EventWriter<SpawnWallEvent>,
    mut paddles_query: Query<(Entity, &Paddle, &mut Fade), With<Collider>>,
) {
    for GoalEliminatedEvent(eliminated_goal) in event_reader.iter() {
        // Fade out the paddle for the eliminated goal.
        for (entity, paddle, mut fade) in paddles_query.iter_mut() {
            if paddle.goal_side != *eliminated_goal {
                continue;
            }

            // Stop the paddle from moving and colliding.
            commands
                .entity(entity)
                .remove::<Movement>()
                .remove::<Collider>();
            fade.fade_out_and_despawn();
            break;
        }

        // Fade in the wall for the eliminated goal.
        spawn_wall_events.send(SpawnWallEvent {
            goal_side: eliminated_goal.clone(),
            is_instant: false,
        });
    }
}

/// Fades out any existing `Wall` entities.
pub fn goal_despawn_walls(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Fade), With<Wall>>,
) {
    for (entity, mut fade) in query.iter_mut() {
        commands.entity(entity).remove::<Collider>();
        fade.fade_out_and_despawn();
    }
}
