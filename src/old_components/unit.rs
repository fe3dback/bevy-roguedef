use bevy::prelude::{Component, Name, Reflect, Sprite, Transform};

use crate::game::energy::CmpEnergyContainer;

#[derive(Component, Reflect, Default)]
#[require(Transform, Sprite, Name, EUnitType)]
pub struct CmpUnit {}

#[derive(Component, Default, Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
pub enum EUnitType {
    #[default]
    Object,
    Creature,
    Building,
}

impl EUnitType {
    pub fn is_creature(&self) -> bool {
        matches!(self, EUnitType::Creature)
    }

    pub fn is_building(&self) -> bool {
        matches!(self, EUnitType::Building)
    }
}
