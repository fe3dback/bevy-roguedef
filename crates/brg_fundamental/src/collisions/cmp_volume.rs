use bevy::prelude::{Component, Reflect};
use brg_core::prelude::types::Meter;

#[derive(Component, Copy, Clone, Reflect)]
pub enum CmpCollisionVolume {
    Circle(Meter),
}
