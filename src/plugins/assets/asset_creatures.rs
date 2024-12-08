use bevy::prelude::{Asset, Image, TypePath};
use ron_asset_manager::prelude::RonAsset;
use ron_asset_manager::Shandle;
use serde::Deserialize;

#[derive(Asset, TypePath, RonAsset, Deserialize, Debug)]
pub struct AssetCreature {
    pub name: String,

    #[asset]
    pub movement: AssetCreatureMovement,

    #[asset]
    pub agent: AssetCreatureAgent,
}

#[derive(RonAsset, Deserialize, Default, Debug)]
pub struct AssetCreatureMovement {
    pub speed: f32,
}

#[derive(RonAsset, Deserialize, Default, Debug)]
pub struct AssetCreatureAgent {
    #[asset]
    pub sprite: Shandle<Image>,
}