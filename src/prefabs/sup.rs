use crate::plugins::assets::asset::GameAssets;
use crate::plugins::assets::asset_creatures::AssetCreature;
use bevy::ecs::system::SystemParam;
use bevy::prelude::{Assets, Commands, Res};

#[derive(SystemParam)]
pub struct SupPrefabs<'w, 's> {
    pub(super) cmd: Commands<'w, 's>,

    pub(super) assets: Res<'w, GameAssets>,
    pub(super) assets_creature: Res<'w, Assets<AssetCreature>>,
}
