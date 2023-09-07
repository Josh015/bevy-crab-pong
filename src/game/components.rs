use crate::prelude::*;

/// A component for a corner barrier entity that exists only to deflect
/// [`Ball`] entities.
#[derive(Component)]
pub struct Barrier;

/// Marks a [`Goal`] entity so that [`Paddle`] and [`Wall`] entities can use it
/// as a parent, and so [`Ball`] entities can score against it.
#[derive(Component)]
pub struct Goal;

/// A component that makes a paddle that can deflect [`Ball`] entities and
/// moves left->right and vice versa along a single axis when [`Collider`].
#[derive(Clone, Component, Eq, PartialEq, Debug, Hash)]
pub struct Paddle;

/// The ball being targeted by AI paddles.
#[derive(Component)]
pub struct Target(pub Entity);

/// Assigns and entity to a given side of the arena.
#[derive(Clone, Component, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

impl Side {
    /// Perpendicular distance from a given goal to a ball's edge.
    ///
    /// Positive distances for inside the arena, negative for out of bounds.
    pub fn distance_to_ball(&self, ball_transform: &GlobalTransform) -> f32 {
        let ball_translation = ball_transform.translation();

        match *self {
            Self::Top => GOAL_HALF_WIDTH + ball_translation.z - BALL_RADIUS,
            Self::Right => GOAL_HALF_WIDTH - ball_translation.x - BALL_RADIUS,
            Self::Bottom => GOAL_HALF_WIDTH - ball_translation.z - BALL_RADIUS,
            Self::Left => GOAL_HALF_WIDTH + ball_translation.x - BALL_RADIUS,
        }
    }

    /// Get the (+/-)(X/Z) axis the side occupies.
    pub fn axis(&self) -> Vec3 {
        match *self {
            Self::Top => -Vec3::Z,
            Self::Right => Vec3::X,
            Self::Bottom => Vec3::Z,
            Self::Left => -Vec3::X,
        }
    }

    /// Map a ball's global position to a side's local x-axis.
    pub fn get_ball_position(&self, ball_transform: &GlobalTransform) -> f32 {
        match *self {
            Self::Top => -ball_transform.translation().x,
            Self::Right => -ball_transform.translation().z,
            Self::Bottom => ball_transform.translation().x,
            Self::Left => ball_transform.translation().z,
        }
    }
}

#[derive(Clone, Component, Copy, Default, PartialEq, Debug)]
pub enum Team {
    #[default]
    Enemies,
    Allies,
}

/// A component that makes an entity a wall in a [`Goal`] that can deflect
/// [`Ball`] entities away from the entire goal when [`Collider`].
#[derive(Component)]
pub struct Wall;

/// A component for an animated textured water plane.
#[derive(Component, Default)]
pub struct Ocean {
    pub scroll: f32,
}

/// A component that causes a camera to sway back and forth in a slow
/// reciprocating motion as it focuses on the origin.
#[derive(Component)]
pub struct SwayingCamera;

/// A component for marking a [`Text`] UI entity as displaying the hit points
/// for an associated [`Goal`].
#[derive(Component)]
pub struct HitPointsUi;

#[derive(Bundle, Default)]
pub struct FadeBundle {
    pub fade_animation: FadeAnimation,
    pub fade: Fade,
}

/// A component that specifies the entity's fade effect animation.
#[derive(Clone, Component, Copy, Default, PartialEq, Debug)]
pub enum FadeAnimation {
    /// Uses [`StandardMaterial`] color and alpha blending to show/hide entity.
    ///
    /// When paired with [`Fade::In`] the entity's [`StandardMaterial`] must
    /// first be set to [`AlphaMode::Blend`] and have its color alpha set to
    /// zero to avoid visual popping.
    #[default]
    Opacity,

    /// Uses [`Transform`] scale to grow/shrink the entity.
    ///
    /// When paired with [`Fade::In`] the entity's [`Transform`] scale must
    /// first be set to EPSILON to avoid visual popping. We can't use zero
    /// since that prevents it from appearing at all.
    Scale {
        /// The maximum scale to start/end with when fading out/in.
        max_scale: Vec3,

        /// Use either 0/1 to remove/mark an axis for the scale effect.
        axis_mask: Vec3,
    },
}

/// A component that makes an entity fade in/out and then despawn if needed.
#[derive(Clone, Component, Copy, PartialEq, Debug)]
pub enum Fade {
    /// Fade-in effect with a progress value in the range \[0,1\].
    In(f32),

    /// Fade-out effect with a progress value in the range \[0,1\].
    Out(f32),
}

impl Default for Fade {
    fn default() -> Self {
        Self::In(0.0)
    }
}

/// Tags an entity to only exist in the listed game states.
#[derive(Component)]
pub struct ForState<T> {
    pub states: Vec<T>,
}

/// Marks an entity that can be collided with by a [`Ball`] entity.
#[derive(Component)]
pub struct Collider;

/// A component for a ball entity that must have inertia and be able to deflect
/// upon collision when [`Collider`].
#[derive(Component)]
pub struct Ball;

/// A component that marks an entity as being controlled by the keyboard.
#[derive(Component)]
pub struct KeyboardInput;

/// A component that marks an entity as being controlled by AI.
#[derive(Component)]
pub struct AiInput;

/// Whether the entity has positive or negative force acting on it.
#[derive(Component, Clone, Copy, PartialEq)]
pub enum Force {
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

/// Distance from an entity's current position to where it will come to a full
/// stop if it begins decelerating immediately.
#[derive(Component, Clone, Default)]
pub struct StoppingDistance(pub f32);

#[derive(Bundle, Default)]
pub struct VelocityBundle {
    pub heading: Heading,
    pub speed: Speed,
}

#[derive(Bundle, Default)]
pub struct AccelerationBundle {
    pub velocity: VelocityBundle,
    pub max_speed: MaxSpeed,
    pub acceleration: Acceleration,
    pub stopping_distance: StoppingDistance,
}
