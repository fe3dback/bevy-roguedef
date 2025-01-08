use bevy::prelude::{Entity, Event, Handle};
use brg_fundamental::prelude::DtoCollisionHit;
use brg_scene::prelude::{AssetSound, AssetSpell};

use crate::units::cmp_team::ETeam;

#[derive(Event)]
pub struct EvtProjectileCollided {
    pub projectile_entity: Entity,
    pub projectile_team:   ETeam,
    pub target_entity:     Entity,
    pub target_team:       ETeam,
    pub collision:         DtoCollisionHit,
    pub cast:              Handle<AssetSpell>,
    pub sound:             Option<Handle<AssetSound>>,
}
