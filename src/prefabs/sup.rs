use bevy::ecs::system::SystemParam;
use bevy::prelude::{Assets, Commands, Res};
use bevy_sprite3d::Sprite3dParams;

use crate::plugins::assets::asset::GameAssets;
use crate::plugins::assets::asset_creatures::AssetCreature;
use crate::plugins::assets::asset_game::AssetGame;

#[derive(SystemParam)]
pub struct SupPrefabs<'w, 's> {
    pub(super) cmd: Commands<'w, 's>,

    pub(super) assets:           Res<'w, GameAssets>,
    pub(super) assets_game:      Res<'w, Assets<AssetGame>>,
    pub(super) assets_creatures: Res<'w, Assets<AssetCreature>>,
    pub(super) sprite_params:    Sprite3dParams<'w, 's>,
}
