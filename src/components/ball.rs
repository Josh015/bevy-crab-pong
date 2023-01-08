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

/// Automatically spawns [`Ball`] entities from the center of the arena.
pub fn spawn_balls(
    config: Res<GameConfig>,
    run_state: Res<RunState>,
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

    // TODO: Figure out how to give each ball own material without constantly
    // creating more?
    let material = materials.add(StandardMaterial {
        alpha_mode: AlphaMode::Blend,
        base_color: Color::rgba(1.0, 1.0, 1.0, 0.0),
        ..default()
    });

    let entity = commands
        .spawn((
            Ball,
            ForState {
                states: vec![GameScreen::Playing, GameScreen::Paused],
            },
            FadeBundle::default(),
            PbrBundle {
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
            },
        ))
        .id();

    info!("Ball({:?}) -> Spawning", entity);
}

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameScreen::Playing).with_system(spawn_balls),
        );
    }
}
