use bevy::prelude::{Component, Reflect, Transform};

use crate::components::lib::V2;

#[derive(Component, Reflect, Default)]
#[require(Transform)]
pub struct CmpTransform2D {
    pub position: V2,
    pub height:   f32,
    pub angle:    f32,
}
