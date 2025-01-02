use bevy::asset::Asset;
use bevy::prelude::TypePath;
use ron_asset_manager::prelude::RonAsset;
use serde::Deserialize;

#[derive(Asset, RonAsset, TypePath, Deserialize, Debug)]
pub struct AssetHeightMapData {
    pub width:  u32,
    pub height: u32,
    pub points: Vec<f32>,
}
