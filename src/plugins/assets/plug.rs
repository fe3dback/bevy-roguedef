use {
    crate::plugins::{
        assets::{asset::GameAssets, asset_creatures::AssetCreature, asset_game::AssetGame},
        core::state::enums::GameState,
    },
    bevy::{app::App, prelude::Plugin},
    bevy_asset_loader::prelude::*,
    ron_asset_manager::RonAssetPlugin,
};

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
            );
    }
}
