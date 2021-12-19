use super::*;
use bevy::{math::Vec3, prelude::Transform};

/// A component that makes an entity a wall in a `Goal` that can deflect `Ball`
/// entities away from the entire goal when `Active`.
#[derive(Component)]
pub struct Wall;

impl Wall {
    pub const DIAMETER: f32 = 0.05;
    pub const HEIGHT: f32 = 0.1;
    pub const RADIUS: f32 = 0.5 * Wall::DIAMETER;
    pub const SCALE: Vec3 =
        const_vec3!([ARENA_WIDTH, Wall::DIAMETER, Wall::DIAMETER]);
}

/// Makes a `Wall` entity `Active` at the very start of a `Fade::In` so that it
/// can immediately deflect balls before its animation has finished.
pub fn begin_fade_system(
    mut commands: Commands,
    query: Query<(Entity, &Fade), (With<Wall>, Added<Fade>)>,
) {
    for (entity, fade) in query.iter() {
        if matches!(*fade, Fade::In(_)) {
            commands.entity(entity).insert(Active);
        }
    }
}

/// Handles `Wall` `Fade` animations by making them shrink along their width
/// into a pancake just before vanishing entirely.
pub fn fade_animation_system(
    mut query: Query<(&Fade, &mut Transform), With<Wall>>,
) {
    for (fade, mut transform) in query.iter_mut() {
        let x_mask = fade.opacity();
        let yz_mask = x_mask.powf(0.001);

        transform.scale = Wall::SCALE * Vec3::new(x_mask, yz_mask, yz_mask);
    }
}
