use super::*;
use crate::GameConfig;
use bevy::{ecs::prelude::*, math::Vec3, prelude::Transform};

#[derive(Component)]
pub struct Wall;

pub fn start_fade_system(
    mut commands: Commands,
    query: Query<(Entity, &Fade), (With<Wall>, Added<Fade>)>,
) {
    for (entity, fade) in query.iter() {
        // Immediately activate walls to avoid repeating eliminated animation
        if matches!(*fade, Fade::In(_)) {
            commands.entity(entity).insert(Active);
        }
    }
}

pub fn step_fade_animation_system(
    config: Res<GameConfig>,
    mut query: Query<(&mut Transform, &Fade), With<Wall>>,
) {
    // Wall shrinks along its width into a pancake and then vanishes
    for (mut transform, fade) in query.iter_mut() {
        let x_mask = fade.opacity();
        let yz_mask = x_mask.powf(0.001);

        transform.scale =
            config.wall_scale() * Vec3::new(x_mask, yz_mask, yz_mask);
    }
}
