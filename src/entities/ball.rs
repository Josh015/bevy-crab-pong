#![allow(clippy::type_complexity)]

use crate::prelude::*;

pub const BALL_DIAMETER: f32 = 0.08;
pub const BALL_HEIGHT: f32 = 0.05;
pub const BALL_RADIUS: f32 = 0.5 * BALL_DIAMETER;

/// Marks an entity that can be collided with by a [`Ball`] entity.
#[derive(Component)]
pub struct Collider;

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

/// A basic reflect function that also normalizes the result.
pub fn reflect(d: Vec3, n: Vec3) -> Vec3 {
    (d - (2.0 * (d.dot(n) * n))).normalize()
}

/// Checks if multiple [`Ball`] entities have collided with each other.
fn ball_to_ball_collisions(
    mut commands: Commands,
    balls_query: Query<
        (Entity, &GlobalTransform, &Heading),
        (With<Ball>, With<Collider>),
    >,
) {
    for (entity, ball_transform, ball_heading) in &balls_query {
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
                || ball_heading.0.dot(axis) <= 0.0
            {
                continue;
            }

            // Deflect the ball away from the other ball.
            commands
                .entity(entity)
                .insert(Heading(reflect(ball_heading.0, axis)));

            info!("Ball({:?}): Collided Ball({:?})", entity, entity2);
            break;
        }
    }
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

// TODO: Debug option to directly control single ball's exact position with
// keyboard and see how paddles respond. Can go in goals, triggering a score and
// ball return?

// TODO: Add debug visualizations for bounding shapes.

// TODO: Need a fix for the rare occasion when a ball just bounces infinitely
// between two walls in a straight line? Maybe make all bounces slightly adjust
// ball angle rather than pure reflection?

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BallResources>()
            .add_systems(
                Update,
                debug_ball_paths.in_set(GameSystemSet::Debugging),
            )
            .add_systems(
                PostUpdate,
                ball_to_ball_collisions.in_set(GameSystemSet::Collision),
            );
    }
}
