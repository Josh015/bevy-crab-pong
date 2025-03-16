use bevy::prelude::*;

/// The direction in which the entity is moving.
#[derive(Clone, Component, Debug)]
pub struct Direction(pub Dir3);

impl Default for Direction {
    fn default() -> Self {
        Self(Dir3::NEG_Z)
    }
}

impl From<Vec3> for Direction {
    fn from(value: Vec3) -> Self {
        Direction(Dir3::new_unchecked(value.normalize()))
    }
}

impl Direction {
    pub fn reflect(direction: &Direction, axis: Vec3) -> Self {
        let i = *direction.0;
        let n = axis;
        let r = i - (2.0 * (i.dot(n) * n));

        Direction::from(r)
    }
}
