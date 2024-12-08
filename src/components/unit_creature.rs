use {
    crate::components::{movement::CmpMovement, unit::CmpUnit},
    bevy::prelude::{Component, Reflect},
};

#[derive(Component, Reflect, Default)]
#[require(CmpUnit, CmpMovement)]
pub struct CmpUnitCreature {}
