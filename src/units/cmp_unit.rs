use bevy::prelude::{Component, Reflect};
use brg_fundamental::prelude::CmpTransform2D;

use super::stats::health::cmp_health::CmpHealth;

// CmpUnit is base class for all dynamic game objects with health (units, creatures, buildings, etc...)
#[derive(Component, Debug, Reflect, Default)]
#[require(CmpTransform2D, CmpHealth)]
pub struct CmpUnit {}
