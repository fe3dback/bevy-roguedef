use bevy::prelude::*;
use brg_fundamental::prelude::*;
use brg_scene::prelude::InGame;

use super::sup_prefabs::SupPrefabs;

impl<'w, 's> SupPrefabs<'w, 's> {
    pub(crate) fn example_scene(&mut self) {
        self.cmd.spawn((
            Name::from("example #plane"),
            StateScoped(InGame),
            CmpTransform2D {
                yaw: std::f32::consts::FRAC_PI_2,
                rotation_kind: TransformRotationKind::YPointOnPosZ,
                ..default()
            },
            Mesh3d(self.basic_meshes.add(Circle::new(4.0))),
            MeshMaterial3d(self.materials.add(StandardMaterial {
                base_color_texture: Some(self.assets.texture_placeholder1.clone()),
                double_sided: true,
                ..default()
            })),
        ));
        self.cmd.spawn((
            Name::from("example #cube"),
            StateScoped(InGame),
            CmpTransform2D {
                height: 0.5,
                ..default()
            },
            Mesh3d(self.basic_meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(self.materials.add(StandardMaterial {
                base_color_texture: Some(self.assets.texture_placeholder2.clone()),

                ..default()
            })),
        ));
    }
}
