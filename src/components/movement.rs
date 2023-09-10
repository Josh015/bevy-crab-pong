use bevy::prelude::{Bundle, Component, Vec3};

/// Whether the entity has positive or negative force acting on it.
#[derive(Clone, Component, Copy, Debug, Eq, Hash, PartialEq)]
#[component(storage = "SparseSet")]
pub enum Force {
    Positive,
    Negative,
}

/// The normalized direction vector along which the entity will move.
#[derive(Clone, Component, Debug, Default)]
pub struct Heading(pub Vec3);

/// The current speed of this entity.
#[derive(Clone, Component, Debug, Default)]
pub struct Speed(pub f32);

/// The maximum speed this entity can reach after accelerating.
#[derive(Clone, Component, Debug, Default)]
pub struct MaxSpeed(pub f32);

/// The `max_speed / seconds_to_reach_max_speed`.
#[derive(Clone, Component, Debug, Default)]
pub struct Acceleration(pub f32);

/// Distance from an entity's current position to where it will come to a full
/// stop if it begins decelerating immediately.
#[derive(Clone, Component, Debug, Default)]
pub struct StoppingDistance(pub f32);

/// Marks an entity as moving with a fixed speed and direction.
#[derive(Bundle, Clone, Debug, Default)]
pub struct VelocityBundle {
    pub heading: Heading,
    pub speed: Speed,
}

/// Marks an entity that accelerates and decelerates when [`Force`] is applied.
#[derive(Bundle, Clone, Debug, Default)]
pub struct AccelerationBundle {
    pub velocity: VelocityBundle,
    pub max_speed: MaxSpeed,
    pub acceleration: Acceleration,
    pub stopping_distance: StoppingDistance,
}
