use bevy::prelude::{Handle, Image, Resource};
use bevy_asset_loader::prelude::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "textures/placeholders/grid_blueprint.ktx2")]
    pub texture_placeholder1: Handle<Image>,

    #[asset(path = "textures/placeholders/grid_orange.ktx2")]
    pub texture_placeholder2: Handle<Image>,
    // #[asset(path = "data/data.game.ron")]
    // pub game: Handle<AssetGame>,
    // #[asset(path = "data/creatures", collection(typed, mapped))]
    // pub creatures: HashMap<String, Handle<AssetCreature>>,
    //
    // #[asset(path = "sprites", collection(typed, mapped))]
    // pub sprites:                  HashMap<String, Handle<Image>>,
}
