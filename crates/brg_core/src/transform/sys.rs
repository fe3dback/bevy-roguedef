use bevy::math::Quat;
use bevy::prelude::{Query, Transform};

use super::cmp::CmpTransform2D;

pub fn transform_apply(mut query: Query<(&CmpTransform2D, &mut Transform)>) {
    for (trm2d, mut trm3d) in query.iter_mut() {
        trm3d.translation = trm2d.position.as_3d();
        trm3d.translation.z = trm2d.height;
        trm3d.rotation = Quat::from_rotation_z(trm2d.angle);
    }
}
