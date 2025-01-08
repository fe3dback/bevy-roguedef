use bevy::math::bounding::BoundingVolume;
use bevy::prelude::{Component, Isometry2d, Reflect};
use brg_core::prelude::types::Meter;

#[derive(Component, Copy, Clone, Reflect)]
pub enum CmpCollisionVolume {
    Circle(Meter),
}
