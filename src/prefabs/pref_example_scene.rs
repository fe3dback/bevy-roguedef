use bevy::prelude::*;
use brg_core::prelude::V2;
use brg_fundamental::prelude::*;
use brg_scene::prelude::InGame;

use super::sup_prefabs::SupPrefabs;

impl<'w, 's> SupPrefabs<'w, 's> {
    pub(crate) fn example_scene(&mut self) {
        self.cmd.spawn((
            Name::from("example #plane"),
            StateScoped(InGame),
            CmpTransform2D {
                position: V2::new(0.0, 0.0),
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
                position: V2::new(0.0, 0.0),
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

    pub(crate) fn example_terrain(&mut self) {
        let hm_data_asset = self
            .assets_hmdata
            .get(&self.assets.terrain_placeholder_hm_data)
            .unwrap();

        let hm_data = CmpExternalHeightmapDataImporter {
            width:  hm_data_asset.width,
            height: hm_data_asset.height,
            points: hm_data_asset.points.clone(),
        };

        self.cmd.spawn((
            StateScoped(InGame),
            Name::from("example #terrain"),
            CmpTransform2D {
                position: V2::new(0.0, 0.0),
                yaw: std::f32::consts::FRAC_PI_2,
                rotation_kind: TransformRotationKind::YPointOnPosZ,
                height_kind: TransformHeightKind::Absolute,
                ..default()
            },
            SceneRoot(self.assets.terrain_placeholder.clone()),
            MeshMaterial3d(self.materials.add(StandardMaterial {
                base_color_texture: Some(self.assets.texture_placeholder1.clone()),
                double_sided: true,
                ..default()
            })),
            CmpTerrainMarkerMesh,
            hm_data,
        ));
    }
}
