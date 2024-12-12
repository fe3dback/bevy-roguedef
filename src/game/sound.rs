use {
    crate::{
        components::{lib::V2, transform::CmpTransform2D},
        game::common::CmpTimeToLife,
    },
    bevy::{audio::Volume, ecs::system::SystemParam, prelude::*},
    rand_chacha::{rand_core::RngCore, ChaCha8Rng},
    std::time::Duration,
};

#[derive(Resource)]
pub struct ResRandomSoundSource {
    pub rnd: ChaCha8Rng,
}

#[derive(SystemParam)]
pub struct SupSounds<'w, 's> {
    cmd:          Commands<'w, 's>,
    asset_server: Res<'w, AssetServer>,
    rand:         ResMut<'w, ResRandomSoundSource>,
}

impl<'w, 's> SupSounds<'w, 's> {
    pub fn play_shot(&mut self, pos: V2) {
        self.spawn_audio(
            pos,
            vec!["pl_gun1", "pl_gun2"],
            Duration::from_millis(200),
            0.35,
        );
    }

    pub fn play_impact(&mut self, pos: V2) {
        self.spawn_audio(
            pos,
            vec!["bullet_hit1", "bullet_hit2"],
            Duration::from_millis(300),
            0.8,
        );
    }

    pub fn play_reload(&mut self, pos: V2) {
        self.spawn_audio(
            pos,
            vec!["reload2", "reload3"],
            Duration::from_millis(2000),
            0.8,
        );
    }

    fn spawn_audio(&mut self, pos: V2, files: Vec<&str>, ttl: Duration, volume: f32) {
        let random_file = self.rand.rnd.next_u32() % files.len() as u32;
        let sound = files[random_file as usize].to_string();

        self.cmd.spawn((
            Name::from(format!("sounds/{}", sound)),
            CmpTransform2D {
                position: pos,
                angle:    0.0,
            },
            CmpTimeToLife {
                seconds_left: ttl.as_secs_f32(),
            },
            AudioPlayer::new(self.asset_server.load(format!("sounds/{}.wav", sound))),
            PlaybackSettings::ONCE
                .with_spatial(true)
                .with_volume(Volume::new(volume)),
        ));
    }
}
