use bevy::app::App;
use bevy::prelude::{in_state, AssetApp, IntoSystemConfigs, OnEnter, Plugin, Update};

use super::asset_level::{AssetLevel, AssetLevelLoader};
use super::assets_mgas::loader::AssetMGALoader;
use super::assets_mgas::AssetMGA;
use super::evt_on_load::EvtOnLoad;
use super::res_loading_state::ResLoadingState;
use super::res_storage::ResAssetsStorage;
use super::sys_loadscreen::{sys_check_loading_status, sys_load_assets, sys_spawn_loading_screen};
use super::sys_on_load::sys_on_load;
use crate::prelude::{GameState, GameSystemSet};

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            //
            .register_asset_loader(AssetLevelLoader)
            .init_asset::<AssetLevel>()
            .init_asset::<AssetMGA>()
            .init_asset_loader::<AssetMGALoader>()
            .insert_resource(ResAssetsStorage::default())
            .insert_resource(ResLoadingState::default())
            .add_event::<EvtOnLoad>()
            .add_systems(
                OnEnter(GameState::Loading),
                (sys_spawn_loading_screen, sys_load_assets)
                    .in_set(GameSystemSet::ALLOW_ON_LOAD__LoadingAssets),
            )
            .add_systems(
                Update,
                (sys_check_loading_status, sys_on_load)
                    .in_set(GameSystemSet::ALLOW_ON_LOAD__LoadingAssets)
                    .run_if(in_state(GameState::Loading)),
            )
        //-
        ;
    }
}
