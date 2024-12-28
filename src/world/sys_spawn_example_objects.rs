use bevy::prelude::{
    default,
    Assets,
    Circle,
    Color,
    Commands,
    Cuboid,
    Mesh,
    Mesh3d,
    MeshMaterial3d,
    ResMut,
    StandardMaterial,
    StateScoped,
};
use brg_core::prelude::{CmpTransform2D, V2};
use brg_scene::prelude::InGame;

pub fn spawn_example_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn((
        StateScoped(InGame),
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        CmpTransform2D {
            position: V2::new(0.0, 0.0),
            yaw: std::f32::consts::FRAC_PI_2,
            rotation_kind: brg_core::prelude::TransformRotationKind::YPointOnPosZ,
            ..default()
        },
    ));
    // cube
    commands.spawn((
        StateScoped(InGame),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        CmpTransform2D {
            position: V2::new(0.0, 0.0),
            height: 0.5,
            ..default()
        },
    ));
}
