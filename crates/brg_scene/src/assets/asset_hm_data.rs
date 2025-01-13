use std::collections::HashMap;

use bevy::asset::Asset;
use bevy::prelude::TypePath;
use brg_core::prelude::{Area, Chunk, Tile};
use ron_asset_manager::prelude::RonAsset;
use serde::{Deserialize, Serialize};

#[derive(Asset, RonAsset, TypePath, Deserialize, Debug)]
pub struct AssetHeightMapData {
    pub width:  u32,
    pub height: u32,
    pub points: Vec<f32>,
}

#[derive(Asset, TypePath, Serialize, Deserialize, Debug)]
pub struct AssetHeightMapArea {
    pub area:    Area,
    pub heights: Vec<f32>,
}
