use crate::prelude::*;

pub const BALL_DIAMETER: f32 = 0.08;
pub const BALL_HEIGHT: f32 = 0.05;
pub const BALL_RADIUS: f32 = 0.5 * BALL_DIAMETER;
pub const BALL_SPAWNER_POSITION: Vec3 = Vec3::new(
    ARENA_CENTER_POINT.x,
    ARENA_CENTER_POINT.y + BALL_HEIGHT,
    ARENA_CENTER_POINT.z,
);

/// A component for a ball entity that must have inertia and be able to deflect
/// upon collision when [`Collider`].
#[derive(Component)]
pub struct Ball;

/// Cached ball materials and meshes.
#[derive(Debug, Resource)]
pub struct BallResources {
    pub ball_mesh_handle: Handle<Mesh>,
    // pub ball_material_handles: Vec<Handle<StandardMaterial>>,
    // pub next_ball_material_index: usize,
}

impl FromWorld for BallResources {
    fn from_world(world: &mut World) -> Self {
        let ball_mesh_handle = {
            let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
            meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.5,
                sectors: 30,
                stacks: 30,
            }))
        };

        // let max_ball_count =
        //     { world.get_resource::<GameConfig>().unwrap().max_ball_count };

        // let ball_material_handles = {
        //     let mut materials = world
        //         .get_resource_mut::<Assets<StandardMaterial>>()
        //         .unwrap();

        //     (0..max_ball_count)
        //         .into_iter()
        //         .map(|_| {
        //             materials.add(StandardMaterial {
        //                 alpha_mode: AlphaMode::Blend,
        //                 base_color: Color::rgba(1.0, 1.0, 1.0, 0.0),
        //                 ..default()
        //             })
        //         })
        //         .collect()
        // };

        Self {
            ball_mesh_handle,
            // ball_material_handles,
            // next_ball_material_index: 0,
        }
    }
}

/// Automatically spawns [`Ball`] entities from the center of the arena.
fn spawn_balls(
    config: Res<GameConfig>,
    /* mut */ resources: ResMut<BallResources>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    new_balls_query: Query<
        (Entity, Option<&Fade>),
        (With<Ball>, Without<Heading>, Without<Speed>),
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

        commands.entity(entity).insert((
            Collider,
            VelocityBundle {
                heading: Heading(Vec3::new(angle.cos(), 0.0, angle.sin())),
                speed: Speed(config.ball_max_speed),
                ..default()
            },
        ));
        info!("Ball({:?}) -> Launched", entity);
    }

    // Spawn new balls until max is reached.
    if all_balls_query.iter().count() >= config.max_ball_count {
        return;
    }

    let entity = commands
        .spawn((
            Ball,
            ForState {
                states: vec![GameScreen::Playing, GameScreen::Paused],
            },
            FadeBundle::default(),
            PbrBundle {
                mesh: resources.ball_mesh_handle.clone(),
                // TODO: Come up with a solution that doesn't require constant
                // allocation!
                material: materials.add(StandardMaterial {
                    alpha_mode: AlphaMode::Blend,
                    base_color: Color::rgba(1.0, 1.0, 1.0, 0.0),
                    ..default()
                }),
                // material: resources.ball_material_handles
                //     [resources.next_ball_material_index]
                //     .clone(),
                transform: Transform::from_matrix(
                    Mat4::from_scale_rotation_translation(
                        Vec3::splat(BALL_DIAMETER),
                        Quat::IDENTITY,
                        BALL_SPAWNER_POSITION,
                    ),
                ),
                ..default()
            },
        ))
        .id();

    // resources.next_ball_material_index += 1;
    // resources.next_ball_material_index %=
    // resources.ball_material_handles.len();
    info!("Ball({:?}) -> Spawning", entity);
}

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BallResources>()
            .add_system(spawn_balls.in_set(OnUpdate(GameScreen::Playing)));
    }
}
