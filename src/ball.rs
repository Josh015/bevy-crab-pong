use crate::prelude::*;

pub const BALL_DIAMETER: f32 = 0.08;
pub const BALL_HEIGHT: f32 = 0.05;
pub const BALL_RADIUS: f32 = 0.5 * BALL_DIAMETER;

/// A component for a ball entity that must have inertia and be able to deflect
/// upon collision when `Collider`.
#[derive(Component)]
pub struct Ball;

/// Handles the `Fade` animation for a `Ball` entity by causing its material to
/// smoothly blend from transparent->opaque and vice versa.
pub fn ball_fade_animation_system(
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(&Fade, &Handle<StandardMaterial>), With<Ball>>,
) {
    for (fade, material) in query.iter() {
        let material = materials.get_mut(material).unwrap();
        let opacity = fade.opacity();

        material.base_color.set_a(opacity);
        material.alpha_mode = if opacity < 1.0 {
            AlphaMode::Blend
        } else {
            AlphaMode::Opaque
        };
    }
}
