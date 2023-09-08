use crate::components::goals::Side;
use bevy::prelude::*;
use std::collections::HashMap;

/// Assets that need to remain loaded at all times.
#[derive(Debug, Resource)]
pub struct GameCachedAssets {
    pub font_handle: Handle<Font>,
    pub ball_mesh_handle: Handle<Mesh>,
    pub paddle_mesh_handle: Handle<Mesh>,
    pub paddle_material_handles: HashMap<Side, Handle<StandardMaterial>>,
    pub wall_mesh_handle: Handle<Mesh>,
    pub wall_material_handle: Handle<StandardMaterial>,
}

impl FromWorld for GameCachedAssets {
    fn from_world(world: &mut World) -> Self {
        let font_handle = {
            let asset_server = world.get_resource::<AssetServer>().unwrap();

            asset_server.load("fonts/FiraSans-Bold.ttf")
        };
        let (ball_mesh_handle, paddle_mesh_handle, wall_mesh_handle) = {
            let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();

            (
                meshes.add(Mesh::from(shape::UVSphere {
                    radius: 0.5,
                    sectors: 30,
                    stacks: 30,
                })),
                meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            )
        };
        let (paddle_material_handles, wall_material_handle) = {
            let mut materials = world
                .get_resource_mut::<Assets<StandardMaterial>>()
                .unwrap();

            (
                HashMap::from([
                    (Side::Top, materials.add(Color::ORANGE.into())),
                    (Side::Right, materials.add(Color::BLUE.into())),
                    (Side::Bottom, materials.add(Color::RED.into())),
                    (Side::Left, materials.add(Color::PURPLE.into())),
                ]),
                materials.add(Color::hex("00A400").unwrap().into()),
            )
        };

        Self {
            font_handle,
            ball_mesh_handle,
            paddle_mesh_handle,
            paddle_material_handles,
            wall_mesh_handle,
            wall_material_handle,
        }
    }
}

pub struct CachedAssetsPlugin;

impl Plugin for CachedAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameCachedAssets>();
    }
}
