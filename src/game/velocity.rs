use bevy::{ecs::prelude::*, prelude::*};

#[derive(Component)]
pub struct Velocity {
    pub speed: f32,
    pub direction: Vec3,
}

pub fn movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity)>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation +=
            velocity.direction * (velocity.speed * time.delta_seconds());
    }
}
