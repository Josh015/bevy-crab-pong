use bevy::prelude::*;
use std::ops::Sub;

// Reflects an incidence vector around a normal vector.
pub fn reflect(i: Vec3, n: Vec3) -> Vec3 {
    i - (2.0 * (i.dot(n) * n))
}

/// Decelerate speed by a delta speed.
pub fn decelerate_speed(speed: f32, delta_speed: f32) -> f32 {
    let s = speed.abs().sub(delta_speed).max(0.0);
    speed.max(-s).min(s) // clamp() panics when min == max.
}

/// Get a deflection direction for a position within a range.
pub fn calculate_deflection(delta: f32, width: f32, axis: Vec3) -> Vec3 {
    let rotation_away_from_center = Quat::from_rotation_y(
        std::f32::consts::FRAC_PI_4 * (delta / (0.5 * width)).clamp(-1.0, 1.0),
    );
    let deflection_direction = rotation_away_from_center * -axis;

    deflection_direction
}
