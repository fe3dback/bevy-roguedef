use bevy::prelude::{Component, Reflect, Transform};
use brg_core::prelude::{Angle, Meter, V2};

#[derive(Component, Reflect, Default)]
#[require(Transform)]
pub struct CmpTransform2D {
    pub position: V2,
    pub height:   Meter,
    pub angle:    Angle,
}
