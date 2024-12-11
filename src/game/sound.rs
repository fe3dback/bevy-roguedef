use {
    crate::{
        components::{lib::V2, transform::CmpTransform2D},
        game::common::CmpTimeToLife,
    },
    bevy::{audio::Volume, ecs::system::SystemParam, prelude::*},
    std::time::Duration,
};

#[derive(SystemParam)]
pub struct SupSounds<'w, 's> {
    cmd:          Commands<'w, 's>,
    asset_server: Res<'w, AssetServer>,
}

impl<'w, 's> SupSounds<'w, 's> {
    pub fn play_shot(&mut self, pos: V2) {
        self.spawn_audio(
            pos,
            "shot",
            String::from("sounds/mmsl_shot.ogg"),
            Duration::from_millis(200),
            0.15,
        );
    }

    pub fn play_impact(&mut self, pos: V2) {
        self.spawn_audio(
            pos,
            "impact",
            String::from("sounds/mmsl_impact.ogg"),
            Duration::from_millis(300),
            1.0,
        );
    }

    pub fn play_reload(&mut self, pos: V2) {
        self.spawn_audio(
            pos,
            "reload",
            String::from("sounds/mmsl_reload.ogg"),
            Duration::from_millis(2000),
            0.6,
        );
    }

    fn spawn_audio(
        &mut self,
        pos: V2,
        ent_name: &str,
        filename: String,
        ttl: Duration,
        volume: f32,
    ) {
        self.cmd.spawn((
            Name::from(format!("audio/{}", ent_name)),
            CmpTransform2D {
                position: pos,
                angle:    0.0,
            },
            CmpTimeToLife {
                seconds_left: ttl.as_secs_f32(),
            },
            AudioPlayer::new(self.asset_server.load(filename)),
            PlaybackSettings::ONCE
                .with_spatial(true)
                .with_volume(Volume::new(volume)),
        ));
    }
}
