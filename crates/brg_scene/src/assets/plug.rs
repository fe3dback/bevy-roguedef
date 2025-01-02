use bevy::app::App;
use bevy::prelude::Plugin;
use bevy_asset_loader::prelude::*;
use ron_asset_manager::RonAssetPlugin;

use super::asset::GameAssets;
use crate::assets::asset_hm_data::AssetHeightMapData;
use crate::prelude::{AssetCreature, GameState};

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .add_plugins(RonAssetPlugin::<AssetHeightMapData>::create("hmdata.ron"))
            .add_plugins(RonAssetPlugin::<AssetCreature>::create("creature.ron"))
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    .continue_to_state(GameState::InGame { paused: false })
                    .load_collection::<GameAssets>(),
            )
            // .init_asset_loader::<AssetLdtkCircuitLoader>()
            // .init_asset::<AssetLdtkCircuit>()
        //-
        ;
    }
}
