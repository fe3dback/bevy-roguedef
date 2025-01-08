use bevy::prelude::EventReader;
use brg_fundamental::prelude::SupSound;

use super::evt::EvtProjectileCollided;

pub fn play_sound_on_collide(mut sound: SupSound, mut reader: EventReader<EvtProjectileCollided>) {
    for evt in reader.read() {
        let Some(fx) = evt.sound.clone() else {
            continue;
        };

        sound.play_spatial(evt.collision.pos, fx);
    }
}
