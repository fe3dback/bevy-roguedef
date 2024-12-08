use crate::components::transform::CmpTransform2D;
use bevy::prelude::{Quat, Query, Transform};

pub fn transform_apply(
    mut query: Query<(&CmpTransform2D, &mut Transform)>,
) {
    for (trm2d, mut trm3d) in query.iter_mut() {
        trm3d.translation = trm2d.position.as_3d();
        trm3d.rotation = Quat::from_rotation_z(trm2d.angle);
    }
}