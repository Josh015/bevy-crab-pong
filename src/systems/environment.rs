use bevy::prelude::*;

use crate::{
    components::environment::{Ocean, SwayingCamera},
    constants::*,
    serialization::{GameAssets, GameConfig},
};

use super::GameSystemSet;

fn make_camera_slowly_sway_back_and_forth(
    game_assets: Res<GameAssets>,
    game_configs: Res<Assets<GameConfig>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<SwayingCamera>, With<Camera3d>)>,
) {
    let game_config = game_configs.get(&game_assets.game_config).unwrap();
    let mut transform = query.single_mut();
    let x = (time.elapsed_seconds() * game_config.swaying_camera_speed).sin()
        * GOAL_HALF_WIDTH;

    *transform = Transform::from_xyz(x * 0.5, 2.0, 1.5)
        .looking_at(FIELD_CENTER_POINT, Vec3::Y);
}

fn animate_ocean_with_scrolling_texture_effect(
    game_assets: Res<GameAssets>,
    game_configs: Res<Assets<GameConfig>>,
    time: Res<Time>,
    mut query: Query<(&mut Ocean, &mut Transform)>,
) {
    // FIXME: Translate the plane on the Z-axis, since we currently can't
    // animate the texture coordinates.
    let game_config = game_configs.get(&game_assets.game_config).unwrap();
    let (mut animated_water, mut transform) = query.single_mut();

    *transform = Transform::from_xyz(0.0, -0.01, animated_water.scroll);

    animated_water.scroll +=
        game_config.animated_water_speed * time.delta_seconds();
    animated_water.scroll %= 1.0;
}

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                make_camera_slowly_sway_back_and_forth,
                animate_ocean_with_scrolling_texture_effect,
            )
                .in_set(GameSystemSet::Environment),
        );
    }
}
