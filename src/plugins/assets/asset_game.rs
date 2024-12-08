use crate::plugins::assets::asset_creatures::AssetCreature;
use bevy::prelude::{Asset, TypePath};
use ron_asset_manager::prelude::RonAsset;
use ron_asset_manager::Shandle;
use serde::Deserialize;

#[derive(Asset, TypePath, RonAsset, Deserialize, Debug)]
pub struct AssetGame {
    pub player: String,
}