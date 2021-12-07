use crate::GameConfig;
use bevy::{
    core::Time,
    prelude::{Component, Query, Res, Transform},
};

/// A component for an animated textured water plane.
#[derive(Component, Default)]
pub struct AnimatedWater {
    scroll: f32,
}

/// Scrolls a material's texture.
pub fn animation_system(
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
