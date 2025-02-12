use bevy::prelude::{Component, Reflect, Transform};
use brg_core::prelude::types::{Angle, Meter};
use brg_core::prelude::{V2, V3};

#[derive(Default, Reflect, Debug)]
pub enum TransformRotationKind {
    #[default]
    NormalYUp,
    YPointOnPosZ,
}

#[derive(Default, Reflect, Debug)]
pub enum TransformHeightKind {
    #[default]
    AboveTerrain,
    Absolute,
}

#[derive(Default, Reflect, Debug)]
pub enum TransformMasterSlave {
    #[default]
    /// map all trm values into bevy transform component
    OwnTransformIsMaster,

    /// support only basic mapping of position and height
    BevyTransformIsMaster,
}

#[derive(Component, Reflect, Default, Debug)]
#[require(Transform)]
pub struct CmpTransform2D {
    pub master:               TransformMasterSlave,
    pub position:             V2,
    pub origin_visual_offset: V3,
    pub height:               Meter,
    pub height_kind:          TransformHeightKind,
    pub angle:                Angle,
    pub yaw:                  Angle,
    pub roll:                 Angle,
    pub rotation_kind:        TransformRotationKind,
}
