use {
    crate::plugins::assets::{asset_creatures::AssetCreature, asset_game::AssetGame},
    bevy::{
        prelude::{Handle, Res, Resource},
        utils::HashMap,
    },
    bevy_asset_loader::prelude::AssetCollection,
};

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "data/data.game.ron")]
    pub game: Handle<AssetGame>,

    #[asset(path = "data/creatures", collection(typed, mapped))]
    pub creatures: HashMap<String, Handle<AssetCreature>>,
}
