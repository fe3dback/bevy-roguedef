use {
    crate::{
        components::{movement::CmpMovement, unit::CmpUnit},
        game::damage::CmpHealth,
    },
    bevy::prelude::{Component, Reflect},
};

#[derive(Component, Reflect, Default)]
#[require(CmpUnit, CmpMovement, CmpHealth)]
pub struct CmpUnitCreature {}
