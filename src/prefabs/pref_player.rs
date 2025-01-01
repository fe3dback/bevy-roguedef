use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::{default, Capsule3d, Mesh3d, Name};
use brg_core::prelude::types::Speed;
use brg_core::prelude::{CmpTransform2D, V2, V3};

use crate::prefabs::sup::SupPrefabs;
use crate::units::cmp_unit_creature::CmpUnitMovementInput;
use crate::units::player::cmp_marker_player::CmpMarkerPlayer;
use crate::world::camera::cmp::CmpMarkerCameraTarget;

impl<'w, 's> SupPrefabs<'w, 's> {
    pub(crate) fn player(
        &mut self,
    ) -> (
        Name,
        Mesh3d,
        MeshMaterial3d<StandardMaterial>,
        CmpTransform2D,
        CmpMarkerPlayer,
        CmpUnitMovementInput,
        CmpMarkerCameraTarget,
    ) {
        (
            Name::from("Player"),
            Mesh3d(self.meshes.add(Capsule3d::new(0.35, 1.4))),
            MeshMaterial3d(self.materials.add(StandardMaterial {
                base_color_texture: Some(self.assets.texture_placeholder2.clone()),

                ..default()
            })),
            CmpTransform2D {
                position: V2::new(0.0, 0.0),
                origin_visual_offset: V3::new(0.0, 0.0, 1.1),
                height: 0.0,
                ..default()
            },
            CmpMarkerPlayer,
            CmpUnitMovementInput {
                speed: Speed::KMH(12.0),
                ..default()
            },
            CmpMarkerCameraTarget,
        )
    }
}
