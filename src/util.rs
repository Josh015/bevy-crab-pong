use bevy::prelude::*;

/// Reflects a vector around and axis and normalizes the result.
pub fn reflect(d: Vec3, n: Vec3) -> Vec3 {
    (d - (2.0 * (d.dot(n) * n))).normalize()
}
