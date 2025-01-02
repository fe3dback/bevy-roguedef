use bevy::prelude::{Entity, Event, Handle};
use brg_scene::prelude::AssetWeapon;

#[derive(Event)]
pub struct EvtWeaponReload {
    pub owner:  Entity,
    pub weapon: Handle<AssetWeapon>,
}
