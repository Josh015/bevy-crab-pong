use crate::prelude::*;

pub const ARENA_CENTER_POINT: Vec3 = Vec3::ZERO;
pub const BALL_SPAWNER_POSITION: Vec3 = Vec3::new(0.0, BALL_HEIGHT, 0.0);

/// A component that causes a camera to sway back and forth in a slow
/// reciprocating motion as it focuses on the origin.
#[derive(Component, Default)]
pub struct SwayingCamera;

/// A component for an animated textured water plane.
#[derive(Component, Default)]
pub struct AnimatedWater {
    pub scroll: f32,
}

/// Handles setting up the static arena entities.
pub fn spawn_arena_system(
    mut run_state: ResMut<RunState>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spawn_wall_events: EventWriter<SpawnWallEvent>,
) {
    let unit_plane = meshes.add(Mesh::from(shape::Plane { size: 1.0 }));

    // Cameras
    commands
        .spawn_bundle(Camera3dBundle::default())
        .insert(SwayingCamera::default());

    // Light
    let light_transform = Mat4::from_euler(
        EulerRot::ZYX,
        0.0,
        std::f32::consts::FRAC_PI_4,
        -std::f32::consts::FRAC_PI_4,
    );
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            shadow_projection: OrthographicProjection {
                left: -10.0,
                right: 10.0,
                bottom: -10.0,
                top: 10.0,
                near: -50.0,
                far: 50.0,
                ..default()
            },
            // shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_matrix(light_transform),
        ..default()
    });

    // Ocean
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("257AFFCC").unwrap(),
                alpha_mode: AlphaMode::Blend,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, -0.01, 0.0),
            ..default()
        })
        .insert(AnimatedWater::default());

    // Beach
    commands.spawn_bundle(PbrBundle {
        mesh: unit_plane.clone(),
        material: materials.add(Color::hex("C4BD99").unwrap().into()),
        transform: Transform::from_matrix(
            Mat4::from_scale_rotation_translation(
                Vec3::splat(GOAL_WIDTH),
                Quat::IDENTITY,
                ARENA_CENTER_POINT,
            ),
        ),
        ..default()
    });

    // Goals
    let unit_cube = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let barrier_material = materials.add(Color::hex("750000").unwrap().into());
    let goal_configs = [
        (
            Side::Bottom,
            UiRect {
                bottom: Val::Px(5.0),
                right: Val::Px(400.0),
                ..default()
            },
        ),
        (
            Side::Right,
            UiRect {
                top: Val::Px(400.0),
                right: Val::Px(5.0),
                ..default()
            },
        ),
        (
            Side::Top,
            UiRect {
                top: Val::Px(5.0),
                left: Val::Px(400.0),
                ..default()
            },
        ),
        (
            Side::Left,
            UiRect {
                bottom: Val::Px(400.0),
                left: Val::Px(5.0),
                ..default()
            },
        ),
    ];

    for (i, (side, rect)) in goal_configs.iter().enumerate() {
        // Walls
        spawn_wall_events.send(SpawnWallEvent {
            side: side.clone(),
            is_instant: true,
        });

        // Goals
        commands
            .spawn_bundle(PbrBundle {
                transform: Transform::from_rotation(Quat::from_axis_angle(
                    Vec3::Y,
                    std::f32::consts::TAU
                        * (i as f32 / goal_configs.len() as f32),
                ))
                .mul_transform(Transform::from_xyz(0.0, 0.0, GOAL_HALF_WIDTH)),
                ..default()
            })
            .insert_bundle((Goal, side.clone()))
            .with_children(|parent| {
                // Barrier
                parent
                    .spawn_bundle(PbrBundle {
                        mesh: unit_cube.clone(),
                        material: barrier_material.clone(),
                        transform: Transform::from_matrix(
                            Mat4::from_scale_rotation_translation(
                                Vec3::new(
                                    BARRIER_DIAMETER,
                                    BARRIER_HEIGHT,
                                    BARRIER_DIAMETER,
                                ),
                                Quat::IDENTITY,
                                Vec3::new(
                                    GOAL_HALF_WIDTH,
                                    0.5 * BARRIER_HEIGHT,
                                    0.0,
                                ),
                            ),
                        ),
                        ..default()
                    })
                    .insert_bundle((Barrier, Collider));
            });

        // Score
        commands
            .spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    position: *rect,
                    ..default()
                },
                text: Text::from_section(
                    "",
                    TextStyle {
                        font: run_state.font_handle.clone(),
                        font_size: 50.0,
                        color: Color::RED,
                    },
                ),
                ..default()
            })
            .insert_bundle((side.clone(), HitPointsUi));

        run_state.goals_hit_points.insert(side.clone(), 0);
    }
}

/// Makes a [`SwayingCamera`] entity slowly sway back and forth.
pub fn arena_swaying_camera_system(
    config: Res<GameConfig>,
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Camera3d>, With<SwayingCamera>)>,
) {
    let mut transform = query.single_mut();
    let x = (time.time_since_startup().as_secs_f32()
        * config.swaying_camera_speed)
        .sin()
        * GOAL_HALF_WIDTH;

    *transform = Transform::from_xyz(x * 0.5, 2.0, 1.5)
        .looking_at(ARENA_CENTER_POINT, Vec3::Y);
}

/// Scrolls a material's texture.
pub fn arena_animated_water_system(
    config: Res<GameConfig>,
    time: Res<Time>,
    mut query: Query<(&mut AnimatedWater, &mut Transform)>,
) {
    // FIXME: Translate the plane on the Z-axis, since we currently can't
    // animate the texture coordinates.
    let (mut animated_water, mut transform) = query.single_mut();

    *transform = Transform::from_xyz(0.0, -0.01, animated_water.scroll);

    animated_water.scroll += config.animated_water_speed * time.delta_seconds();
    animated_water.scroll %= 1.0;
}

/// Automatically spawns [`Ball`] entities from the center of the arena.
pub fn arena_ball_spawner_system(
    config: Res<GameConfig>,
    run_state: Res<RunState>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    new_balls_query: Query<
        (Entity, Option<&Fade>),
        (With<Ball>, Without<Movement>),
    >,
    all_balls_query: Query<&Ball>,
) {
    // Check for any non-moving new balls.
    for (entity, fade) in &new_balls_query {
        // Pause the spawning process until the new ball finishes fading in.
        if fade.is_some() {
            return;
        }

        // Make the ball collidable and launch it in a random direction.
        let mut rng = SmallRng::from_entropy();
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);

        commands.entity(entity).insert_bundle((
            Collider,
            Movement {
                direction: Vec3::new(angle.cos(), 0.0, angle.sin()),
                speed: config.ball_starting_speed,
                max_speed: config.ball_max_speed,
                acceleration: config.ball_max_speed
                    / config.ball_seconds_to_max_speed,
                delta: Some(MovementDelta::Positive),
            },
        ));
        info!("Ball({:?}) -> Launched", entity);
    }

    // Spawn new balls until max is reached.
    if all_balls_query.iter().count() >= config.max_ball_count {
        return;
    }

    // TODO: Figure out how to give each ball own material without constantly creating more?
    let material = materials.add(StandardMaterial {
        alpha_mode: AlphaMode::Blend,
        base_color: Color::rgba(1.0, 1.0, 1.0, 0.0),
        ..default()
    });

    let entity = commands
        .spawn_bundle(PbrBundle {
            mesh: run_state.ball_mesh_handle.clone(),
            material: material.clone(),
            transform: Transform::from_matrix(
                Mat4::from_scale_rotation_translation(
                    Vec3::splat(BALL_DIAMETER),
                    Quat::IDENTITY,
                    BALL_SPAWNER_POSITION,
                ),
            ),
            ..default()
        })
        .insert_bundle(FadeBundle::default())
        .insert_bundle((
            Ball,
            ForState {
                states: vec![AppState::Game, AppState::Pause],
            },
        ))
        .id();

    info!("Ball({:?}) -> Spawning", entity);
}

/// Checks if a [`Ball`] has collided with a compatible entity, and then
/// deflects it away from the point of impact.
pub fn arena_collision_system(
    mut commands: Commands,
    balls_query: Query<
        (Entity, &GlobalTransform, &Movement),
        (With<Ball>, With<Collider>),
    >,
    paddles_query: Query<(&Side, &Transform), (With<Paddle>, With<Collider>)>,
    barriers_query: Query<&GlobalTransform, (With<Barrier>, With<Collider>)>,
    walls_query: Query<&Side, (With<Wall>, With<Collider>)>,
) {
    for (entity, ball_transform, movement) in &balls_query {
        let ball_direction = movement.direction;

        // Ball collisions
        for (entity2, transform2, _) in &balls_query {
            // Prevent balls from colliding with themselves.
            if entity == entity2 {
                continue;
            }

            let ball_to_ball_distance = ball_transform
                .translation()
                .distance(transform2.translation());
            let axis = (transform2.translation()
                - ball_transform.translation())
            .normalize();

            // Check that the ball is touching the other ball and facing it.
            if ball_to_ball_distance > 2.0 * BALL_RADIUS
                || ball_direction.dot(axis) <= 0.0
            {
                continue;
            }

            // Deflect the ball away from the other ball.
            let mut new_movement = movement.clone();

            new_movement.direction = reflect(ball_direction, axis);
            commands.entity(entity).insert(new_movement);

            info!("Ball({:?}) -> Collided Ball({:?})", entity, entity2);
            break;
        }

        // Paddle collisions
        for (side, transform) in &paddles_query {
            let axis = side.axis();
            let ball_distance = side.distance_to_ball(ball_transform);
            let ball_local_position =
                side.map_ball_position_to_paddle_local_space(ball_transform);
            let ball_to_paddle = transform.translation.x - ball_local_position;
            let distance_from_paddle_center = (ball_to_paddle).abs();

            // Check that the ball is touching the paddle and facing the goal.
            if ball_distance > PADDLE_HALF_DEPTH
                || distance_from_paddle_center >= PADDLE_HALF_WIDTH
                || ball_direction.dot(axis) <= 0.0
            {
                continue;
            }

            // Reverse the ball's direction and rotate it outward based on how
            // far its position is from the paddle's center.
            let rotation_away_from_center = Quat::from_rotation_y(
                std::f32::consts::FRAC_PI_4
                    * (ball_to_paddle / PADDLE_HALF_WIDTH),
            );
            let mut new_movement = movement.clone();

            new_movement.direction =
                rotation_away_from_center * -ball_direction;
            commands.entity(entity).insert(new_movement);

            info!("Ball({:?}) -> Collided Paddle({:?})", entity, side);
            break;
        }

        // Barrier collisions
        for barrier_transform in &barriers_query {
            let ball_to_barrier_distance = ball_transform
                .translation()
                .distance(barrier_transform.translation());

            // Prevent balls from deflecting through the floor.
            let mut axis =
                barrier_transform.translation() - ball_transform.translation();

            axis.y = 0.0;
            axis = axis.normalize();

            // Check that the ball is touching the barrier and facing it.
            if ball_to_barrier_distance > BARRIER_RADIUS + BALL_RADIUS
                || ball_direction.dot(axis) <= 0.0
            {
                continue;
            }

            // Deflect the ball away from the barrier.
            let mut new_movement = movement.clone();

            new_movement.direction = reflect(ball_direction, axis);
            commands.entity(entity).insert(new_movement);

            info!("Ball({:?}) -> Collided Barrier", entity);
            break;
        }

        // Wall collisions
        for side in &walls_query {
            let ball_distance = side.distance_to_ball(ball_transform);
            let axis = side.axis();

            // Check that the ball is touching and facing the wall.
            if ball_distance > WALL_RADIUS || ball_direction.dot(axis) <= 0.0 {
                continue;
            }

            // Deflect the ball away from the wall.
            let mut new_movement = movement.clone();

            new_movement.direction = reflect(ball_direction, axis);
            commands.entity(entity).insert(new_movement);

            info!("Ball({:?}) -> Collided Wall({:?})", entity, side);
            break;
        }
    }
}

/// A basic reflect function that also normalizes the result.
fn reflect(d: Vec3, n: Vec3) -> Vec3 {
    (d - (2.0 * (d.dot(n) * n))).normalize()
}
