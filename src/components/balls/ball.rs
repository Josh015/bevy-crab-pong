#![allow(clippy::type_complexity)]

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

        Self { ball_mesh_handle }
    }
}

/// Automatically spawns [`Ball`] entities from the center of the arena.
fn spawn_balls(
    run_state: Res<RunState>,
    config: Res<GameConfig>,
    resources: ResMut<BallResources>,
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
                speed: Speed(config.ball_speed),
            },
        ));
        info!("Ball({:?}): Launched", entity);
    }

    // Spawn new balls until max is reached.
    if all_balls_query.iter().count()
        >= config.modes[run_state.mode_index].max_ball_count
    {
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
                material: materials.add(StandardMaterial {
                    alpha_mode: AlphaMode::Blend,
                    base_color: Color::rgba(1.0, 1.0, 1.0, 0.0),
                    ..default()
                }),
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

    info!("Ball({:?}): Spawning", entity);
}

// TODO: Make this work with all object movement, not just Balls?
fn debug_ball_paths(
    query: Query<(&GlobalTransform, &Heading), (With<Ball>, Without<Fade>)>,
    mut gizmos: Gizmos,
) {
    for (global_transform, heading) in &query {
        gizmos.line(
            global_transform.translation(),
            global_transform.translation() + heading.0 * 20.0,
            Color::RED,
        )
    }
}

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BallResources>().add_systems(
            Update,
            (
                spawn_balls.in_set(GameSystemSet::GameplayLogic),
                debug_ball_paths.in_set(GameSystemSet::Debugging),
            ),
        );
    }
}