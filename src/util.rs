use bevy::prelude::*;

// Reflects an incidence vector around a normal vector.
pub fn reflect(i: Vec3, n: Vec3) -> Vec3 { i - (2.0 * (i.dot(n) * n)) }
