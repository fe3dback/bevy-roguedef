use bevy::prelude::{Component, Reflect, Transform};

use crate::prelude::types::{Angle, Meter};
use crate::prelude::{V2, V3};

#[derive(Default, Reflect)]
pub enum TransformRotationKind {
    #[default]
    NormalYUp,
    YPointOnPosZ,
}

#[derive(Default, Reflect)]
pub enum TransformHeightKind {
    #[default]
    AboveTerrain,
    Absolute,
}

#[derive(Component, Reflect, Default)]
#[require(Transform)]
pub struct CmpTransform2D {
    pub position:             V2,
    pub origin_visual_offset: V3,
    pub height:               Meter,
    pub height_kind:          TransformHeightKind,
    pub angle:                Angle,
    pub yaw:                  Angle,
    pub roll:                 Angle,
    pub rotation_kind:        TransformRotationKind,
}
