use bevy::app::App;
use bevy::prelude::{AssetApp, Plugin};
use bevy_asset_loader::prelude::*;
use ron_asset_manager::RonAssetPlugin;

use crate::plugins::assets::asset::GameAssets;
use crate::plugins::assets::asset_creatures::AssetCreature;
use crate::plugins::assets::asset_game::AssetGame;
use crate::plugins::assets::asset_ldtk_circuit::{AssetLdtkCircuit, AssetLdtkCircuitLoader};
use crate::plugins::core::state::enums::GameState;

pub struct Plug {}

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .add_plugins(RonAssetPlugin::<AssetGame>::create("game.ron"))
            .add_plugins(RonAssetPlugin::<AssetCreature>::create("creature.ron"))
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    .continue_to_state(GameState::InGame { paused: false })
                    .load_collection::<GameAssets>(),
            )
            .init_asset_loader::<AssetLdtkCircuitLoader>()
            .init_asset::<AssetLdtkCircuit>();
    }
}
