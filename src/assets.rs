use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::config::GameConfig;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(key = "game.config")]
    pub game_config: Handle<GameConfig>,

    #[asset(key = "fonts.menu")]
    pub font_menu: Handle<Font>,

    #[asset(key = "images.paddle")]
    pub image_paddle: Handle<Image>,
}

/// Assets that need to remain loaded at all times.
#[derive(Debug, Resource)]
pub struct CachedAssets {
    pub ball_mesh: Handle<Mesh>,
    pub paddle_mesh: Handle<Mesh>,
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
        let wall_material = {
            let mut materials = world
                .get_resource_mut::<Assets<StandardMaterial>>()
                .unwrap();

            materials.add(Color::hex("00A400").unwrap().into())
        };

        Self {
            ball_mesh,
            paddle_mesh,
            wall_mesh,
            wall_material,
        }
    }
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CachedAssets>();
    }
}
