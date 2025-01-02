use bevy::audio::AudioSource;
use bevy::prelude::{Asset, TypePath};
use ron_asset_manager::prelude::RonAsset;
use ron_asset_manager::Shandle;
use serde::Deserialize;

#[derive(Asset, TypePath, RonAsset, Deserialize, Debug, Clone)]
pub struct AssetSound {
    #[asset]
    pub samples: Vec<AssetSoundSample>,
}

#[derive(RonAsset, Deserialize, Debug, Clone)]
pub struct AssetSoundSample {
    #[asset]
    pub file:   Shandle<AudioSource>,
    pub volume: f32,
}
