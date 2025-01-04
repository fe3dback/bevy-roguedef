use bevy::prelude::{Assets, EventReader, Query, Res};
use brg_fundamental::prelude::{CmpTransform2D, SupSound};
use brg_scene::prelude::AssetWeapon;

use crate::units::weapon::evt_reload::EvtWeaponReload;
use crate::units::weapon::evt_shot::EvtWeaponShot;

pub fn on_reload(
    mut consumer: EventReader<EvtWeaponReload>,
    owner_query: Query<(&CmpTransform2D)>,
    weapons: Res<Assets<AssetWeapon>>,
    mut sounds: SupSound,
) {
    for ev in consumer.read() {
        let Ok(trm) = owner_query.get(ev.owner) else {
            continue;
        };

        let Some(weapon) = weapons.get(&ev.weapon) else {
            continue;
        };

        let Some(sound) = &weapon.reload_sound else {
            continue;
        };

        sounds.play_spatial(trm.position, sound.handle.clone());
    }
}

pub fn on_shot(
    mut consumer: EventReader<EvtWeaponShot>,
    owner_query: Query<(&CmpTransform2D)>,
    weapons: Res<Assets<AssetWeapon>>,
    mut sounds: SupSound,
) {
    for ev in consumer.read() {
        let Ok(trm) = owner_query.get(ev.owner) else {
            continue;
        };

        let Some(weapon) = weapons.get(&ev.weapon) else {
            continue;
        };

        let Some(sound) = &weapon.fire_sound else {
            continue;
        };

        sounds.play_spatial(trm.position, sound.handle.clone());
    }
}
