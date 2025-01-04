use bevy::prelude::{default, Asset, TypePath};
use ron_asset_manager::prelude::RonAsset;
use ron_asset_manager::Shandle;
use serde::Deserialize;

use crate::prelude::AssetSpell;

#[derive(Asset, TypePath, RonAsset, Deserialize, Debug, Clone)]
pub struct AssetProjectile {
    pub speed_start_mps:           f32,
    pub speed_acceleration_mps:    f32,
    pub collision_radius_m:        f32,
    pub collision_spread_radius_m: f32,
    pub life_time_sec:             f32,
    #[asset]
    pub cast:                      Shandle<AssetSpell>,
}

impl Default for AssetProjectile {
    fn default() -> Self {
        Self {
            speed_start_mps:           1.0,
            speed_acceleration_mps:    0.0,
            collision_radius_m:        0.1,
            collision_spread_radius_m: 0.0,
            life_time_sec:             1.0,
            cast:                      Shandle::<AssetSpell>::default(),
        }
    }
}
