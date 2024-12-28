use bevy::math::EulerRot;
use bevy::prelude::{Quat, Query, Transform};

use super::cmp::CmpTransform2D;
use crate::prelude::TransformRotationKind;

pub fn transform_apply(mut query: Query<(&CmpTransform2D, &mut Transform)>) {
    for (trm2d, mut trm3d) in query.iter_mut() {
        trm3d.translation = trm2d.position.as_3d();
        trm3d.translation.y = trm2d.height;
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
