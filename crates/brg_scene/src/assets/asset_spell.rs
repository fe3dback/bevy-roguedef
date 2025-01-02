use bevy::prelude::{Asset, Image, TypePath};
use ron_asset_manager::prelude::RonAsset;
use ron_asset_manager::Shandle;
use serde::Deserialize;

use crate::prelude::AssetEffect;

#[derive(Asset, TypePath, RonAsset, Deserialize, Debug, Clone)]
pub struct AssetSpell {
    #[asset]
    pub apply_one_time: Shandle<AssetEffect>,
}
