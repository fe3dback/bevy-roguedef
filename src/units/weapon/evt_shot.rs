use bevy::prelude::{Entity, Event, Handle};
use brg_core::prelude::V2;
use brg_scene::prelude::AssetWeapon;

#[derive(Event)]
pub struct EvtWeaponShot {
    pub owner:  Entity,
    pub weapon: Handle<AssetWeapon>,
    pub aim_to: V2,
}
