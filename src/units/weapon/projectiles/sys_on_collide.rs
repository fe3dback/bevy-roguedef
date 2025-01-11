use bevy::prelude::EventReader;
use brg_fundamental::prelude::SupSound;

use super::evt::EvtProjectileCollided;
use crate::units::spells::sup::SupSpells;

pub fn on_collide(
    mut reader: EventReader<EvtProjectileCollided>,
    mut sound: SupSound,
    mut spells: SupSpells,
) {
    for evt in reader.read() {
        let Some(fx) = evt.sound.clone() else {
            continue;
        };

        sound.play_spatial(evt.collision.pos, fx);
        spells.cast_target_spell(evt.caster_entity, evt.target_entity, evt.cast.clone());
    }
}
