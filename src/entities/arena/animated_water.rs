use crate::prelude::*;

/// A component for an animated textured water plane.
#[derive(Component, Default)]
pub struct AnimatedWater {
    pub scroll: f32,
}

/// Scrolls a material's texture.
fn animated_water(
    game_config: Res<GameConfig>,
    time: Res<Time>,
    mut query: Query<(&mut AnimatedWater, &mut Transform)>,
) {
    // FIXME: Translate the plane on the Z-axis, since we currently can't
    // animate the texture coordinates.
    let (mut animated_water, mut transform) = query.single_mut();

    *transform = Transform::from_xyz(0.0, -0.01, animated_water.scroll);

    animated_water.scroll +=
        game_config.animated_water_speed * time.delta_seconds();
    animated_water.scroll %= 1.0;
}

pub struct AnimatedWaterPlugin;

impl Plugin for AnimatedWaterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animated_water);
    }
}
