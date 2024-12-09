use {
    crate::{components::lib::V2, game::teams::Team},
    bevy::prelude::{Entity, Reflect},
};

#[derive(Reflect, Debug, Default, Copy, Clone)]
pub enum DamageKind {
    #[default]
    Melee,
    RangedSimple,
    Fire,
}

#[derive(Reflect, Debug, Copy, Clone)]
pub struct Damage {
    pub amount: f32,
    pub kind:   DamageKind,
}

impl Default for Damage {
    fn default() -> Self {
        Self {
            kind:   DamageKind::default(),
            amount: 1.0,
        }
    }
}

#[derive(Reflect, Debug, Copy, Clone)]
pub struct DamageCastSource {
    pub damage:     Damage,
    pub origin:     V2,
    pub caster:     Option<Entity>,
    pub owner_team: Team,
}
