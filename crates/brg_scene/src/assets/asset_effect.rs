use bevy::prelude::{Asset, TypePath};
use ron_asset_manager::prelude::RonAsset;
use serde::Deserialize;

#[derive(Asset, TypePath, RonAsset, Deserialize, Debug, Clone, Default)]
pub struct AssetEffect {
    #[asset]
    pub damage: Option<AssetEffectDamage>,
}

#[derive(RonAsset, Deserialize, Debug, Clone, Copy)]
pub struct AssetEffectDamage {
    pub allow_friendly_fire: bool,
    pub base:                u32,
    pub dice_count:          u32,
    pub dice_faces:          u32,
}
