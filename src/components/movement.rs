use crate::prelude::*;
use std::ops::Add;

/// Whether the entity has positive, negative, or zero force acting on it.
#[derive(Component, Clone, Copy, Default, PartialEq)]
pub enum Force {
    #[default]
    Zero,
    Positive,
    Negative,
}

/// The normalized direction vector along which the entity will move.
#[derive(Component, Clone, Default)]
pub struct Heading(pub Vec3);

/// The current speed of this entity.
#[derive(Component, Clone, Default)]
pub struct Speed(pub f32);

/// The maximum speed this entity can reach after accelerating.
#[derive(Component, Clone, Default)]
pub struct MaxSpeed(pub f32);

/// The `max_speed / seconds_to_reach_max_speed`.
#[derive(Component, Clone, Default)]
pub struct Acceleration(pub f32);

#[derive(Bundle, Default)]
pub struct VelocityBundle {
    pub heading: Heading,
    pub speed: Speed,
}

#[derive(Bundle, Default)]
pub struct AccelerationBundle {
    pub velocity: VelocityBundle,
    pub force: Force,
    pub max_speed: MaxSpeed,
    pub acceleration: Acceleration,
}

/// Handles calculating the actual acceleration/deceleration over time for
/// entities with [`Force`], [`MaxSpeed`], and [`Acceleration`].
fn acceleration(
    time: Res<Time>,
    mut query: Query<(&mut Speed, &Force, &MaxSpeed, &Acceleration)>,
) {
    for (mut speed, force, max_speed, acceleration) in &mut query {
        let delta_speed = acceleration.0 * time.delta_seconds();

        speed.0 = match force {
            Force::Zero => decelerate_speed(speed.0, delta_speed),
            _ => speed
                .0
                .add(if *force == Force::Positive {
                    delta_speed
                } else {
                    -delta_speed
                })
                .clamp(-max_speed.0, max_speed.0),
        };
    }
}

/// Handles moving entities with [`Heading`] and [`Speed`].
fn velocity(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Heading, &Speed)>,
) {
    for (mut transform, heading, speed) in &mut query {
        transform.translation += heading.0 * (speed.0 * time.delta_seconds());
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (acceleration, velocity)
                .chain()
                .in_set(LogicalSet::Movement),
        );
    }
}
