use bevy::prelude::{Entity, Event, Handle};
use brg_fundamental::prelude::DtoCollisionHit;
use brg_scene::prelude::{AssetSound, AssetSpell};

use crate::units::cmp_team::ETeam;

#[derive(Event)]
#[allow(unused)]
pub struct EvtProjectileCollided {
    pub caster_entity:     Entity,
    pub projectile_entity: Entity,
    pub target_entity:     Entity,
    pub caster_team:       ETeam,
    pub target_team:       ETeam,
    pub collision:         DtoCollisionHit,
    pub cast:              Handle<AssetSpell>,
    pub sound:             Option<Handle<AssetSound>>,
}
