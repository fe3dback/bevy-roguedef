use crate::components::lib::V2;
use bevy::prelude::Transform;
use bevy::prelude::{Component, Reflect};

#[derive(Component, Reflect, Default)]
#[require(Transform)]
pub struct CmpTransform2D {
    pub position: V2,
    pub angle: f32,
}