use bevy::math::EulerRot;
use bevy::prelude::{Quat, Query, Transform};
use brg_core::prelude::{V2, V3};

use super::cmp::CmpTransform2D;
use crate::prelude::{
    SupHeightmap,
    TransformHeightKind,
    TransformMasterSlave,
    TransformRotationKind,
};

pub fn transform_apply(
    mut query: Query<(&mut CmpTransform2D, &mut Transform)>,
    mut hm: SupHeightmap,
) {
    for (mut trm2d, mut trm3d) in query.iter_mut() {
        match trm2d.master {
            TransformMasterSlave::OwnTransformIsMaster => {
                // transfer 2d position
                trm3d.translation = trm2d.position.as_3d();

                // transfer height
                trm3d.translation.y = match trm2d.height_kind {
                    TransformHeightKind::Absolute => trm2d.height,
                    TransformHeightKind::AboveTerrain => {
                        hm.height_at_pos(trm2d.position) + trm2d.height
                    }
                };

                // add visual origin
                trm3d.translation += trm2d.origin_visual_offset.as_3d();

                // set up rotation
                trm3d.rotation = Quat::from_euler(
                    match trm2d.rotation_kind {
                        TransformRotationKind::NormalYUp => EulerRot::XYZ,
                        TransformRotationKind::YPointOnPosZ => EulerRot::XZY,
                    },
                    -trm2d.yaw,
                    trm2d.angle,
                    trm2d.roll,
                );
            }
            TransformMasterSlave::BevyTransformIsMaster => {
                let pos3d = V3::from_3d(trm3d.translation);

                trm2d.position = V2::new(pos3d.x, pos3d.y);
                trm2d.height = match trm2d.height_kind {
                    TransformHeightKind::Absolute => pos3d.h,
                    TransformHeightKind::AboveTerrain => pos3d.h - hm.height_at_pos(trm2d.position),
                };
            }
        };
    }
}
