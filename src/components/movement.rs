use bevy::prelude::{Component, Reflect, Vec2};
use brg_core::prelude::V2;

use crate::components::transform::CmpTransform2D;

#[derive(Component, Reflect)]
#[require(CmpTransform2D)]
pub struct CmpMovement {
    pub ctl_input: V2,  // vector of movement (0;0 - for idle)
    pub speed:     f32, // speed px in per second
}

impl Default for CmpMovement {
    fn default() -> Self {
        CmpMovement {
            ctl_input: V2::ZERO,
            speed:     2.0,
        }
    }
}

#[derive(Component, Reflect)]
pub struct CmpMarkerMovementRestrictInPlayableArea {}
