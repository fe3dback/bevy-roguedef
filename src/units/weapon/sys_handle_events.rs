use bevy::prelude::{Assets, EventReader, Query, Res};
use brg_fundamental::prelude::{CmpTransform2D, SupSound};
use brg_scene::prelude::AssetWeapon;

use crate::units::weapon::evt_reload::EvtWeaponReload;

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
