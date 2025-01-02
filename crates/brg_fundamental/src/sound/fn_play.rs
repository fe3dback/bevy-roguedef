use bevy::audio::Volume;
use bevy::prelude::{default, AudioPlayer, Handle, Name, PlaybackSettings};
use brg_core::prelude::V2;
use brg_scene::prelude::AssetSound;

use super::sup::SupSound;
use crate::prelude::CmpTransform2D;

impl<'w, 's> SupSound<'w, 's> {
    pub fn play_spatial(&mut self, pos: V2, handle: Handle<AssetSound>) {
        let Some(sound) = self.sounds.get(&handle) else {
            return;
        };

        let Some(sample) = self.rnd.rand_element(&sound.samples) else {
            return;
        };

        self.cmd.spawn((
            Name::from(format!("audio #{}", sample.file.path)),
            CmpTransform2D {
                position: pos,
                angle: 0.0,
                ..default()
            },
            AudioPlayer::new(sample.file.handle.clone()),
            PlaybackSettings::DESPAWN
                .with_spatial(true)
                .with_volume(Volume::new(sample.volume * 100.0)),
        ));
    }
}
