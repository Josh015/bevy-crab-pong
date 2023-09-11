use bevy::{prelude::*, utils::HashMap};

use crate::components::goals::Side;

/// Assets that need to remain loaded at all times.
#[derive(Debug, Resource)]
pub struct CachedAssets {
    pub ball_mesh: Handle<Mesh>,
    pub paddle_mesh: Handle<Mesh>,
    pub paddle_materials: HashMap<Side, Handle<StandardMaterial>>,
    pub wall_mesh: Handle<Mesh>,
    pub wall_material: Handle<StandardMaterial>,
}

impl FromWorld for CachedAssets {
    fn from_world(world: &mut World) -> Self {
        let (ball_mesh, paddle_mesh, wall_mesh) = {
            let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();

            (
                meshes.add(Mesh::from(shape::UVSphere {
                    radius: 0.5,
                    sectors: 30,
                    stacks: 30,
                })),
                // TODO: Replace with crab model.
                meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            )
        };
        let (paddle_materials, wall_material) = {
            let mut materials = world
                .get_resource_mut::<Assets<StandardMaterial>>()
                .unwrap();

            (
                // TODO: Replace with crab model textures.
                HashMap::from([
                    (Side::Bottom, materials.add(Color::WHITE.into())),
                    (Side::Right, materials.add(Color::WHITE.into())),
                    (Side::Top, materials.add(Color::WHITE.into())),
                    (Side::Left, materials.add(Color::WHITE.into())),
                ]),
                materials.add(Color::hex("00A400").unwrap().into()),
            )
        };

        Self {
            ball_mesh,
            paddle_mesh,
            paddle_materials,
            wall_mesh,
            wall_material,
        }
    }
}

pub struct CachedAssetsPlugin;

impl Plugin for CachedAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CachedAssets>();
    }
}
