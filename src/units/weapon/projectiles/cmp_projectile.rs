use bevy::asset::Handle;
use bevy::prelude::{Component, Reflect};
use brg_core::prelude::types::Speed;
use brg_scene::prelude::AssetSpell;

#[derive(Component, Reflect)]
pub struct CmpProjectile {
    pub speed:          Speed,
    pub acceleration:   Speed,
    pub hit_spell_cast: Handle<AssetSpell>,
}
