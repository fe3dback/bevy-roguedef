use crate::plugins::assets::asset_creatures::AssetCreature;
use crate::plugins::assets::asset_game::AssetGame;
use bevy::prelude::Res;
use bevy::prelude::{Handle, Resource};
use bevy::utils::HashMap;
use bevy_asset_loader::prelude::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "data/data.game.ron")]
    pub game: Handle<AssetGame>,

    #[asset(path = "data/creatures", collection(typed, mapped))]
    pub creatures: HashMap<String, Handle<AssetCreature>>,
}