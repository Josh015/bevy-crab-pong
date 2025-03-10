use bevy::prelude::*;

use crate::game::state::PausableSet;

use super::movement::{Heading, Movement};

pub(super) struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            circle_to_circle_collisions.in_set(PausableSet),
        );
    }
}

/// Marks an entity as collidable.
#[derive(Component, Debug, Default)]
pub struct Collider;

/// Adds a circular collider shape.
#[derive(Component, Debug)]
pub struct CircleCollider {
    pub radius: f32,
}

fn circle_to_circle_collisions(
    mut commands: Commands,
    balls_query: Query<
        (
            Entity,
            &CircleCollider,
            &GlobalTransform,
            Option<&Heading>,
            Has<Movement>,
        ),
        With<Collider>,
    >,
) {
    for [
        (entity1, circle1, transform1, heading1, has_movement1),
        (entity2, circle2, transform2, heading2, has_movement2),
    ] in balls_query.iter_combinations()
    {
        // Check that both circles are close enough to touch.
        let delta = transform2.translation() - transform1.translation();

        if delta.length() > circle1.radius + circle2.radius {
            continue;
        }

        // Deflect both circles away from each other.
        let axis1 = Vec3::new(delta.x, 0.0, delta.z).normalize();
        let axis2 = -axis1;

        if let Some(heading1) = heading1 {
            if has_movement1 && heading1.0.dot(axis1) > 0.0 {
                commands
                    .entity(entity1)
                    .insert(Heading::reflect(heading1, axis1));

                if has_movement2 {
                    commands.entity(entity2).insert(Heading::from(axis1));
                }
            }
        }

        if let Some(heading2) = heading2 {
            if has_movement2 && heading2.0.dot(axis2) > 0.0 {
                commands
                    .entity(entity2)
                    .insert(Heading::reflect(heading2, axis2));

                if has_movement1 {
                    commands.entity(entity1).insert(Heading::from(axis2));
                }
            }
        }

        info!("Circle({entity1:?}): Collided Circle({entity2:?})");
    }
}
