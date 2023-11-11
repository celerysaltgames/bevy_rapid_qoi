use bevy::prelude::*;
use bevy_rapid_qoi::QOIPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(QOIPlugin)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, spin)
        .run();
}

#[derive(Component)]
struct SpinMarker;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(1.0, 5.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(shape::Cube::new(1.0).into()),
            material: materials.add(asset_server.load("512x512.qoi").into()),
            ..default()
        })
        .insert(SpinMarker);

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(1.5, 1.5, 1.5).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn spin(mut q: Query<&mut Transform, With<SpinMarker>>) {
    let mut cube = q.single_mut();
    cube.rotate_axis(Vec3::Y, 0.02);
}
