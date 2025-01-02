use bevy::prelude::{Asset, Image, TypePath};
use ron_asset_manager::prelude::RonAsset;
use ron_asset_manager::Shandle;
use serde::Deserialize;

#[derive(Asset, TypePath, RonAsset, Deserialize, Debug, Clone)]
pub struct AssetCreature {
    pub name: String,

    #[asset]
    pub movement: AssetCreatureMovement,
}

#[derive(RonAsset, Deserialize, Default, Debug, Clone)]
pub struct AssetCreatureMovement {
    pub speed: f32,
}
