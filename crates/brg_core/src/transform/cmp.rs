use bevy::prelude::{Component, Reflect, Transform};

use crate::prelude::types::{Angle, Meter};
use crate::prelude::V2;

#[derive(Default, Reflect)]
pub enum TransformRotationKind {
    #[default]
    NormalYUp,
    YPointOnPosZ,
}

#[derive(Component, Reflect, Default)]
#[require(Transform)]
pub struct CmpTransform2D {
    pub position:      V2,
    pub height:        Meter,
    pub angle:         Angle,
    pub yaw:           Angle,
    pub roll:          Angle,
    pub rotation_kind: TransformRotationKind,
}
