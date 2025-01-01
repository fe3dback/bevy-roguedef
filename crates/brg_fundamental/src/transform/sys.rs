use bevy::math::EulerRot;
use bevy::prelude::{Quat, Query, Res, Transform};

use super::cmp::CmpTransform2D;
use crate::prelude::{ResHeightmap, TransformHeightKind, TransformRotationKind};

pub fn transform_apply(mut query: Query<(&CmpTransform2D, &mut Transform)>, hm: Res<ResHeightmap>) {
    for (trm2d, mut trm3d) in query.iter_mut() {
        // transfer 2d position
        trm3d.translation = trm2d.position.as_3d();

        // transfer height
        trm3d.translation.y = match trm2d.height_kind {
            TransformHeightKind::Absolute => trm2d.height,
            TransformHeightKind::AboveTerrain => hm.height_at_pos(trm2d.position) + trm2d.height,
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
}
