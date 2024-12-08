use {
    crate::plugins::assets::asset_creatures::AssetCreature,
    bevy::prelude::{Asset, TypePath},
    ron_asset_manager::{prelude::RonAsset, Shandle},
    serde::Deserialize,
};

#[derive(Asset, TypePath, RonAsset, Deserialize, Debug)]
pub struct AssetGame {
    pub player: String,
}
