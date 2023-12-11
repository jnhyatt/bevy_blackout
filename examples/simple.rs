use bevy::prelude::*;
use bevy_blackout::{BlackoutMaterial, BlackoutPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BlackoutPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .run();
}

#[derive(Component)]
struct Player;

fn setup(
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    mut blackout_materials: ResMut<Assets<BlackoutMaterial>>,
    mut commands: Commands,
) {
    let camera = commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 7.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .id();
    let player = commands
        .spawn((
            Player,
            PbrBundle {
                mesh: meshes.add(
                    shape::Capsule {
                        radius: 0.4,
                        depth: 1.0,
                        ..default()
                    }
                    .into(),
                ),
                material: standard_materials.add(default()),
                transform: Transform::from_xyz(0.0, 0.9, 0.0),
                ..default()
            },
        ))
        .id();
    let viz_marker = commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                color: Color::BLACK,
                range: 20.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .id();
    let _ground_plane = commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(shape::Plane::from_size(20.0).into()),
        material: blackout_materials.add(BlackoutMaterial {
            base: StandardMaterial {
                base_color: Color::GREEN,
                ..default()
            },
            extension: default(),
        }),
        ..default()
    });
    let _sun_light = commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::default().looking_to(Vec3::new(1.0, -1.0, 1.0), Vec3::Y),
        ..default()
    });

    // Columns
    let positions = [
        Vec3::new(-5.0, 2.0, -5.0),
        Vec3::new(-5.0, 2.0, 0.0),
        Vec3::new(-5.0, 2.0, 5.0),
        Vec3::new(0.0, 2.0, 5.0),
        Vec3::new(5.0, 2.0, 5.0),
        Vec3::new(5.0, 2.0, 0.0),
        Vec3::new(5.0, 2.0, -5.0),
        Vec3::new(0.0, 2.0, -5.0),
    ];
    let cylinder_mesh = meshes.add(
        shape::Cylinder {
            radius: 0.5,
            height: 4.0,
            ..default()
        }
        .into(),
    );
    let cylinder_material = blackout_materials.add(BlackoutMaterial {
        base: StandardMaterial {
            base_color: Color::RED,
            ..default()
        },
        extension: default(),
    });
    for position in positions {
        commands.spawn(MaterialMeshBundle {
            mesh: cylinder_mesh.clone(),
            material: cylinder_material.clone(),
            transform: Transform::from_translation(position),
            ..default()
        });
    }

    commands.entity(player).add_child(camera);
    commands.entity(player).add_child(viz_marker);
}

fn move_player(
    mut players: Query<&mut Transform, With<Player>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let amount = time.delta_seconds() * 5.0;
    for mut player in &mut players {
        if keys.pressed(KeyCode::A) {
            player.translation -= Vec3::X * amount;
        }
        if keys.pressed(KeyCode::D) {
            player.translation += Vec3::X * amount;
        }
        if keys.pressed(KeyCode::S) {
            player.translation += Vec3::Z * amount;
        }
        if keys.pressed(KeyCode::W) {
            player.translation -= Vec3::Z * amount;
        }
    }
}
