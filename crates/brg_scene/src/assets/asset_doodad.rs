use bevy::prelude::{Asset, TypePath};
use brg_core::prelude::Id;
use ron_asset_manager::prelude::RonAsset;
use serde::Deserialize;

#[derive(Asset, TypePath, RonAsset, Deserialize, Debug, Clone)]
pub struct AssetDoodad {
    pub id: Id,
}
