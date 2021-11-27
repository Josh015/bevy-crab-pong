use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.7, 0.9, 1.0)))
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let unit_plane = meshes.add(Mesh::from(shape::Plane { size: 1.0 }));

    // Ocean
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
        material: materials.add(Color::hex("257AFF").unwrap().into()),
        transform: Transform::from_xyz(0.5, -0.01, 0.5),
        ..Default::default()
    });

    // Sand
    commands.spawn_bundle(PbrBundle {
        mesh: unit_plane.clone(),
        material: materials.add(Color::hex("C4BD99").unwrap().into()),
        transform: Transform::from_xyz(0.5, 0.0, 0.5),
        ..Default::default()
    });

    let unit_cube = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));

    // Barriers
    let barrier_material = materials.add(Color::hex("750000").unwrap().into());
    let barrier_height = 0.1;
    let barrier_scale = Vec3::splat(0.20);

    commands.spawn_bundle(PbrBundle {
        mesh: unit_cube.clone(),
        material: barrier_material.clone(),
        transform: Transform::from_matrix(
            Mat4::from_scale_rotation_translation(
                barrier_scale,
                Quat::IDENTITY,
                Vec3::new(0.0, barrier_height, 0.0),
            ),
        ),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: unit_cube.clone(),
        material: barrier_material.clone(),
        transform: Transform::from_matrix(
            Mat4::from_scale_rotation_translation(
                barrier_scale,
                Quat::IDENTITY,
                Vec3::new(1.0, barrier_height, 0.0),
            ),
        ),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: unit_cube.clone(),
        material: barrier_material.clone(),
        transform: Transform::from_matrix(
            Mat4::from_scale_rotation_translation(
                barrier_scale,
                Quat::IDENTITY,
                Vec3::new(1.0, barrier_height, 1.0),
            ),
        ),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: unit_cube.clone(),
        material: barrier_material.clone(),
        transform: Transform::from_matrix(
            Mat4::from_scale_rotation_translation(
                barrier_scale,
                Quat::IDENTITY,
                Vec3::new(0.0, barrier_height, 1.0),
            ),
        ),
        ..Default::default()
    });

    // Poles
    let pole_material = materials.add(Color::hex("00A400").unwrap().into());
    let pole_height = 0.1;
    let pole_radius = 0.05;
    let pole_scale_x = Vec3::new(1.0, pole_radius, pole_radius);
    let pole_scale_z = Vec3::new(pole_radius, pole_radius, 1.0);

    commands.spawn_bundle(PbrBundle {
        mesh: unit_cube.clone(),
        material: pole_material.clone(),
        transform: Transform::from_matrix(
            Mat4::from_scale_rotation_translation(
                pole_scale_x,
                Quat::IDENTITY,
                Vec3::new(0.5, pole_height, 0.0),
            ),
        ),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: unit_cube.clone(),
        material: pole_material.clone(),
        transform: Transform::from_matrix(
            Mat4::from_scale_rotation_translation(
                pole_scale_z,
                Quat::IDENTITY,
                Vec3::new(1.0, pole_height, 0.5),
            ),
        ),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: unit_cube.clone(),
        material: pole_material.clone(),
        transform: Transform::from_matrix(
            Mat4::from_scale_rotation_translation(
                pole_scale_x,
                Quat::IDENTITY,
                Vec3::new(0.5, pole_height, 1.0),
            ),
        ),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: unit_cube.clone(),
        material: pole_material.clone(),
        transform: Transform::from_matrix(
            Mat4::from_scale_rotation_translation(
                pole_scale_z,
                Quat::IDENTITY,
                Vec3::new(0.0, pole_height, 0.5),
            ),
        ),
        ..Default::default()
    });

    // Crabs
    let crab_scale = Vec3::splat(0.1);
    let crab_height = 0.05;

    // Orange Crab
    commands.spawn_bundle(PbrBundle {
        mesh: unit_cube.clone(),
        material: materials.add(Color::rgb(1.0, 0.5, 0.0).into()),
        transform: Transform::from_matrix(
            Mat4::from_scale_rotation_translation(
                crab_scale,
                Quat::IDENTITY,
                Vec3::new(0.5, crab_height, 0.0),
            ),
        ),
        ..Default::default()
    });

    // Blue Crab
    commands.spawn_bundle(PbrBundle {
        mesh: unit_cube.clone(),
        material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
        transform: Transform::from_matrix(
            Mat4::from_scale_rotation_translation(
                crab_scale,
                Quat::IDENTITY,
                Vec3::new(1.0, crab_height, 0.5),
            ),
        ),
        ..Default::default()
    });

    // Red Crab
    commands.spawn_bundle(PbrBundle {
        mesh: unit_cube.clone(),
        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        transform: Transform::from_matrix(
            Mat4::from_scale_rotation_translation(
                crab_scale,
                Quat::IDENTITY,
                Vec3::new(0.5, crab_height, 1.0),
            ),
        ),
        ..Default::default()
    });

    // Purple Crab
    commands.spawn_bundle(PbrBundle {
        mesh: unit_cube.clone(),
        material: materials.add(Color::rgb(1.0, 0.0, 1.0).into()),
        transform: Transform::from_matrix(
            Mat4::from_scale_rotation_translation(
                crab_scale,
                Quat::IDENTITY,
                Vec3::new(0.0, crab_height, 0.5),
            ),
        ),
        ..Default::default()
    });

    // Scores
    let score_height = 0.0;
    let score_scale = Vec3::splat(0.25);
    let score_material = materials.add(Color::rgb(1.0, 0.0, 0.0).into());

    commands.spawn_bundle(PbrBundle {
        mesh: unit_plane.clone(),
        material: score_material.clone(),
        transform: Transform::from_matrix(
            Mat4::from_scale_rotation_translation(
                score_scale,
                Quat::IDENTITY,
                Vec3::new(0.5, score_height, -0.5),
            ),
        ),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: unit_plane.clone(),
        material: score_material.clone(),
        transform: Transform::from_matrix(
            Mat4::from_scale_rotation_translation(
                score_scale,
                Quat::IDENTITY,
                Vec3::new(1.5, score_height, 0.5),
            ),
        ),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: unit_plane.clone(),
        material: score_material.clone(),
        transform: Transform::from_matrix(
            Mat4::from_scale_rotation_translation(
                score_scale,
                Quat::IDENTITY,
                Vec3::new(0.5, score_height, 1.5),
            ),
        ),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: unit_plane.clone(),
        material: score_material.clone(),
        transform: Transform::from_matrix(
            Mat4::from_scale_rotation_translation(
                score_scale,
                Quat::IDENTITY,
                Vec3::new(-0.5, score_height, 0.5),
            ),
        ),
        ..Default::default()
    });

    // Balls
    let unit_sphere = meshes.add(Mesh::from(shape::Icosphere {
        radius: 0.5,
        subdivisions: 2,
    }));
    let ball_scale = Vec3::splat(0.05);
    let ball_height = 0.1;

    commands.spawn_bundle(PbrBundle {
        mesh: unit_sphere.clone(),
        material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
        transform: Transform::from_matrix(
            Mat4::from_scale_rotation_translation(
                ball_scale,
                Quat::IDENTITY,
                Vec3::new(0.5, ball_height, 0.5),
            ),
        ),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: unit_sphere.clone(),
        material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
        transform: Transform::from_matrix(
            Mat4::from_scale_rotation_translation(
                ball_scale,
                Quat::IDENTITY,
                Vec3::new(0.5, ball_height, 0.5),
            ),
        ),
        ..Default::default()
    });

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
