use bevy::app::App;
use bevy::prelude::Plugin;
use bevy_asset_loader::prelude::*;

use super::asset::GameAssets;
use crate::prelude::GameState;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            // .add_plugins(RonAssetPlugin::<AssetGame>::create("game.ron"))
            // .add_plugins(RonAssetPlugin::<AssetCreature>::create("creature.ron"))
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
