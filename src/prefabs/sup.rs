use {
    crate::plugins::assets::{
        asset::GameAssets,
        asset_creatures::AssetCreature,
        asset_game::AssetGame,
    },
    bevy::{
        ecs::system::SystemParam,
        prelude::{Assets, Commands, Res},
    },
};

#[derive(SystemParam)]
pub struct SupPrefabs<'w, 's> {
    pub(super) cmd: Commands<'w, 's>,

    pub(super) assets:           Res<'w, GameAssets>,
    pub(super) assets_game:      Res<'w, Assets<AssetGame>>,
    pub(super) assets_creatures: Res<'w, Assets<AssetCreature>>,
}
