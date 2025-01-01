use bevy::prelude::{Asset, TypePath};
use ron_asset_manager::prelude::RonAsset;
use serde::Deserialize;

#[derive(Asset, TypePath, RonAsset, Deserialize, Debug)]
pub struct AssetHeightMapData {
    pub width:  u32,
    pub height: u32,
    pub points: Vec<f32>,
}
