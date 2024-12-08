use {
    crate::components::lib::V2,
    bevy::prelude::{Component, Reflect, Transform},
};

#[derive(Component, Reflect, Default)]
#[require(Transform)]
pub struct CmpTransform2D {
    pub position: V2,
    pub angle:    f32,
}
