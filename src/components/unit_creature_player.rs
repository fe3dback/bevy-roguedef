use {
    crate::components::unit_creature::CmpUnitCreature,
    bevy::prelude::{Component, Reflect},
};

#[derive(Component, Reflect, Default)]
#[require(CmpUnitCreature)]
pub struct CmpUnitCreaturePlayer {}
