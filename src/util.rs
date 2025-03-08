use bevy::prelude::*;

// Reflects an incidence vector around a normal vector.
pub fn reflect(i: Vec3, n: Vec3) -> Vec3 {
    i - (2.0 * (i.dot(n) * n))
}

/// Get a deflection direction for a position within a range.
pub fn hemisphere_deflection(delta: f32, width: f32, forward: Vec3) -> Vec3 {
    let rotation_away_from_center = Quat::from_rotation_y(
        std::f32::consts::FRAC_PI_4 * (delta / (0.5 * width)).clamp(-1.0, 1.0),
    );

    rotation_away_from_center * -forward
}
