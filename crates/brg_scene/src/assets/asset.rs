use bevy::prelude::{Handle, Image, Res, Resource};
use bevy::utils::hashbrown::HashMap;
use bevy_asset_loader::prelude::AssetCollection;

use super::asset_creatures::AssetCreature;
use super::asset_doodad::AssetDoodad;
use crate::assets2::asset_level::AssetLevel;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "maps/example/x.land.bin")]
    pub level: Handle<AssetLevel>,

    #[asset(path = "textures/placeholders/grid_blueprint.ktx2")]
    pub texture_placeholder1: Handle<Image>,

    #[asset(path = "textures/placeholders/grid_orange.ktx2")]
    pub texture_placeholder2: Handle<Image>,

    #[asset(path = "data/creatures", collection(typed, mapped))]
    pub creatures: HashMap<String, Handle<AssetCreature>>,

    #[asset(path = "data/doodads", collection(typed, mapped))]
    pub doodads: HashMap<String, Handle<AssetDoodad>>,
}
