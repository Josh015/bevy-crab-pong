use bevy::{
    ecs::prelude::*,
    math::Vec3,
    prelude::{Assets, Handle, StandardMaterial, Transform},
};

use super::fade::Fade;
use crate::GameConfig;

#[derive(Component)]
pub struct Ball;

pub(super) fn step_fade_animation_system(
    config: Res<GameConfig>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<
        (&Handle<StandardMaterial>, &mut Transform, &mut Fade),
        With<Ball>,
    >,
) {
    // Increase/Decrease balls' opacity to show/hide them
    let mut is_prior_fading = false;

    for (material, mut transform, mut fade) in query.iter_mut() {
        let is_current_fading = matches!(*fade, Fade::In(_));

        // Force current ball to wait if other is also fading in
        if is_prior_fading && is_current_fading {
            *fade = Fade::In(0.0);
            continue;
        }

        is_prior_fading = is_current_fading;

        // materials
        //     .get_mut(material)
        //     .unwrap()
        //     .base_color
        //     .set_a(fade.opacity());

        // FIXME: Use scaling until we can get alpha-blending working
        transform.scale = Vec3::splat(fade.opacity() * config.ball_size);
    }
}
