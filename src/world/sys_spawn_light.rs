use bevy::prelude::*;
use brg_core::prelude::V2;
use brg_scene::prelude::Loaded;

// sun brightness 0..100k
const L1_SUN_BRIGHT: f32 = 5000.0;
const L1_SUN_COLOR: Color = Color::srgb(1.0, 0.97, 0.95);
const L1_SUN_ROTATION: V2 = V2::new(5.3, 0.25);

// ambient brightness 0..50k
const L2_AMBIENT_BRIGHT: f32 = 1000.0;
const L2_AMBIENT_COLOR: Color = Color::srgb(0.85, 0.92, 0.8);
const L2_AMBIENT_ROTATION: V2 = V2::new(2.0, 5.5);

pub fn spawn_light(mut cmd: Commands) {
    cmd.spawn((
        StateScoped(Loaded),
        Name::from("Light Sun"),
        DirectionalLight {
            color: L1_SUN_COLOR,
            illuminance: L1_SUN_BRIGHT,
            ..default()
        },
        Transform {
            rotation: Quat::from_euler(EulerRot::XYZ, L1_SUN_ROTATION.x, L1_SUN_ROTATION.y, 0.0),
            ..default()
        },
    ));

    cmd.spawn((
        StateScoped(Loaded),
        Name::from("Light Ambient"),
        DirectionalLight {
            color: L2_AMBIENT_COLOR,
            illuminance: L2_AMBIENT_BRIGHT,
            ..default()
        },
        Transform {
            rotation: Quat::from_euler(
                EulerRot::XYZ,
                L2_AMBIENT_ROTATION.x,
                L2_AMBIENT_ROTATION.y,
                0.0,
            ),
            ..default()
        },
    ));
}
