use bevy::prelude::{Component, Reflect, Transform};

use crate::prelude::types::{Angle, Meter};
use crate::prelude::V2;

#[derive(Component, Reflect, Default)]
#[require(Transform)]
pub struct CmpTransform2D {
    pub position: V2,
    pub height:   Meter,
    pub angle:    Angle,
}
