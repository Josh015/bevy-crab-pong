use crate::prelude::*;

pub const WALL_DIAMETER: f32 = 0.05;
pub const WALL_HEIGHT: f32 = 0.1;
pub const WALL_RADIUS: f32 = 0.5 * WALL_DIAMETER;
pub const WALL_SCALE: Vec3 =
    Vec3::new(GOAL_WIDTH, WALL_DIAMETER, WALL_DIAMETER);

/// A component that makes an entity a wall in a [`Goal`] that can deflect
/// [`Ball`] entities away from the entire goal when [`Collider`].
#[derive(Component)]
pub struct Wall;

/// An event fired when a [`Wall`] needs to be spawned.
pub struct SpawnWallEvent {
    pub side: Side,
    pub is_instant: bool,
}

pub fn spawn_wall_event(
    run_state: Res<RunState>,
    mut commands: Commands,
    mut event_reader: EventReader<SpawnWallEvent>,
    goals_query: Query<(Entity, &Side), With<Goal>>,
) {
    for SpawnWallEvent { side, is_instant } in event_reader.iter() {
        for (entity, matching_side) in &goals_query {
            if *side != *matching_side {
                continue;
            }

            commands.entity(entity).with_children(|parent| {
                parent.spawn((
                    Wall,
                    side.clone(),
                    Collider,
                    FadeBundle {
                        fade_animation: FadeAnimation::Scale {
                            max_scale: WALL_SCALE,
                            axis_mask: Vec3::new(0.0, 1.0, 1.0),
                        },
                        fade: Fade::In(if *is_instant { 1.0 } else { 0.0 }),
                    },
                    PbrBundle {
                        mesh: run_state.wall_mesh_handle.clone(),
                        material: run_state.wall_material_handle.clone(),
                        transform: Transform::from_matrix(
                            Mat4::from_scale_rotation_translation(
                                Vec3::splat(f32::EPSILON),
                                Quat::IDENTITY,
                                Vec3::new(0.0, WALL_HEIGHT, 0.0),
                            ),
                        ),
                        ..default()
                    },
                ));
            });
            break;
        }
    }
}

/// Fades out any existing [`Wall`] entities.
pub fn despawn_walls(
    mut commands: Commands,
    mut fade_out_entity_events: EventWriter<FadeOutEntityEvent>,
    query: Query<Entity, With<Wall>>,
) {
    for entity in &query {
        commands.entity(entity).remove::<Collider>();
        fade_out_entity_events.send(FadeOutEntityEvent(entity));
    }
}

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnWallEvent>()
            .add_system_set(
                SystemSet::on_exit(AppState::StartMenu)
                    .with_system(despawn_walls),
            )
            .add_system(spawn_wall_event);
    }
}
