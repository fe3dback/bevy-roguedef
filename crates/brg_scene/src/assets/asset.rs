use bevy::prelude::{Handle, Image, Res, Resource, Scene};
use bevy::utils::hashbrown::HashMap;
use bevy_asset_loader::prelude::AssetCollection;

use crate::assets::asset_creatures::AssetCreature;
use crate::assets::asset_hm_data::AssetHeightMapData;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "textures/placeholders/grid_blueprint.ktx2")]
    pub texture_placeholder1: Handle<Image>,

    #[asset(path = "textures/placeholders/grid_orange.ktx2")]
    pub texture_placeholder2: Handle<Image>,

    #[asset(path = "mesh/debug/example_level.glb#Scene0")]
    pub terrain_placeholder:         Handle<Scene>,
    #[asset(path = "mesh/debug/example_level.hmdata.ron")]
    pub terrain_placeholder_hm_data: Handle<AssetHeightMapData>,
    // #[asset(path = "data/data.game.ron")]
    // pub game: Handle<AssetGame>,
    #[asset(path = "data/creatures", collection(typed, mapped))]
    pub creatures:                   HashMap<String, Handle<AssetCreature>>,
    //
    // #[asset(path = "sprites", collection(typed, mapped))]
    // pub sprites:                  HashMap<String, Handle<Image>>,
}
