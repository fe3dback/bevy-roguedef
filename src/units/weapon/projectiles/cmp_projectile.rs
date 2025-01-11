use bevy::asset::Handle;
use bevy::prelude::{Component, Entity, Reflect};
use brg_core::prelude::types::Speed;
use brg_scene::prelude::{AssetSound, AssetSpell};

#[derive(Component, Reflect)]
pub struct CmpProjectile {
    pub caster:         Entity,
    pub speed:          Speed,
    pub acceleration:   Speed,
    pub friendly_fire:  bool,
    pub hit_spell_cast: Handle<AssetSpell>,
    pub hit_sound:      Option<Handle<AssetSound>>,
}
