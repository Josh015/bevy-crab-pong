use crate::prelude::*;
use std::ops::Sub;

/// Calculate a new reduced speed value based on delta speed and clamping
/// to zero.
pub fn decelerate_speed(speed: f32, delta_speed: f32) -> f32 {
    let s = speed.abs().sub(delta_speed).max(0.0);
    speed.max(-s).min(s) // clamp() panics when min == max.
}

/// A basic reflect function that also normalizes the result.
pub fn reflect(d: Vec3, n: Vec3) -> Vec3 {
    (d - (2.0 * (d.dot(n) * n))).normalize()
}
