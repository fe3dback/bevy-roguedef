use bevy::app::App;
use bevy::prelude::{AssetApp, Plugin};
use bevy_asset_loader::prelude::*;
use ron_asset_manager::RonAssetPlugin;

use super::asset::GameAssets;
use super::asset_doodad::AssetDoodad;
use super::asset_level::{AssetLevel, AssetLevelLoader};
use crate::prelude::{
    AssetCreature,
    AssetEffect,
    AssetProjectile,
    AssetSound,
    AssetSpell,
    AssetWeapon,
    GameState,
};

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .register_asset_loader(AssetLevelLoader)
            .init_asset::<AssetLevel>()
            //
            .add_plugins(RonAssetPlugin::<AssetSound>::create("sound.ron"))
            .add_plugins(RonAssetPlugin::<AssetEffect>::create("effect.ron"))
            .add_plugins(RonAssetPlugin::<AssetSpell>::create("spell.ron"))
            .add_plugins(RonAssetPlugin::<AssetProjectile>::create("projectile.ron"))
            .add_plugins(RonAssetPlugin::<AssetWeapon>::create("weapon.ron"))
            .add_plugins(RonAssetPlugin::<AssetCreature>::create("creature.ron")) 
            .add_plugins(RonAssetPlugin::<AssetDoodad>::create("dod.ron")) 

            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    .continue_to_state(GameState::Loaded { game_paused: false })
                    .load_collection::<GameAssets>(),
            )
        // .init_asset_loader::<AssetLdtkCircuitLoader>()
        // .init_asset::<AssetLdtkCircuit>()
        //-
        ;
    }
}
