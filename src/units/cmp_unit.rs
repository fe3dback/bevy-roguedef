use bevy::prelude::{Component, Reflect};
use brg_fundamental::prelude::CmpTransform2D;

// CmpUnit is base class for all dynamic game objects with health (units, creatures, buildings, etc...)
#[derive(Component, Debug, Reflect, Default)]
#[require(CmpTransform2D)]
pub struct CmpUnit {}
