use bevy::prelude::{Asset, TypePath};
use ron_asset_manager::prelude::RonAsset;
use ron_asset_manager::Shandle;
use serde::Deserialize;

use crate::prelude::{AssetProjectile, AssetSound};

#[derive(Asset, TypePath, RonAsset, Deserialize, Debug, Clone)]
pub struct AssetWeapon {
    pub magazine_capacity:        u32,
    pub magazine_reload_time_sec: f32,
    pub hit_reload_time_sec:      f32,
    #[asset]
    pub projectile:               Shandle<AssetProjectile>,
    #[asset]
    pub reload_sound:             Option<Shandle<AssetSound>>,
}
