use bevy::prelude::{Asset, TypePath};
use ron_asset_manager::prelude::RonAsset;
use ron_asset_manager::Shandle;
use serde::Deserialize;

use crate::prelude::AssetWeapon;

#[derive(Asset, TypePath, RonAsset, Deserialize, Debug, Clone)]
pub struct AssetCreature {
    pub name: String,

    #[asset]
    pub movement: AssetCreatureMovement,

    #[asset]
    pub weapon: Option<Shandle<AssetWeapon>>,
}

#[derive(RonAsset, Deserialize, Default, Debug, Clone)]
pub struct AssetCreatureMovement {
    pub speed:              f32,
    pub collision_radius_m: f32,
}
