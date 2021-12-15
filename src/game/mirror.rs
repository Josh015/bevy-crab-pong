use bevy::prelude::*;

/// A component placed on an entity that must mirror another entity's
/// `Transform` state.
#[derive(Component)]
pub struct Mirror(pub Entity);

/// Causes a `Mirror` entity to match the `Transform` states of the parent
/// entity who's ID they contain.
pub fn reflect_parent_entities_system(
    mut query: Query<(&Mirror, &mut Transform)>,
    parent_query: Query<&Transform, Without<Mirror>>,
) {
    for (mirror, mut transform) in query.iter_mut() {
        // Look up the associated entity and state that this one must mirror.
        let parent_transform = parent_query.get(mirror.0).unwrap();
        let mut new_translation = parent_transform.translation.clone();

        // HACK: Mirror along the Y-axis.
        new_translation.y = -new_translation.y;

        // Apply parent state to mirror entity.
        transform.translation = new_translation;
        transform.rotation = parent_transform.rotation;
        transform.scale = parent_transform.scale;
    }
}
