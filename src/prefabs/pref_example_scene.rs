use bevy::prelude::*;
use brg_core::prelude::{CmpTransform2D, V2};

use super::sup::SupPrefabs;

impl<'w, 's> SupPrefabs<'w, 's> {
    pub(crate) fn example_scene(
        &mut self,
    ) -> (
        (Mesh3d, MeshMaterial3d<StandardMaterial>, CmpTransform2D),
        (Mesh3d, MeshMaterial3d<StandardMaterial>, CmpTransform2D),
    ) {
        (
            (
                Mesh3d(self.meshes.add(Circle::new(4.0))),
                MeshMaterial3d(self.materials.add(StandardMaterial {
                    base_color_texture: Some(self.assets.texture_placeholder1.clone()),
                    double_sided: true,
                    ..default()
                })),
                CmpTransform2D {
                    position: V2::new(0.0, 0.0),
                    yaw: std::f32::consts::FRAC_PI_2,
                    rotation_kind: brg_core::prelude::TransformRotationKind::YPointOnPosZ,
                    ..default()
                },
            ),
            (
                Mesh3d(self.meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                MeshMaterial3d(self.materials.add(StandardMaterial {
                    base_color_texture: Some(self.assets.texture_placeholder2.clone()),

                    ..default()
                })),
                CmpTransform2D {
                    position: V2::new(0.0, 0.0),
                    height: 0.5,
                    ..default()
                },
            ),
        )
    }
}
