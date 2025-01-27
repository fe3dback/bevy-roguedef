use bevy::prelude::ResMut;
use bevy::utils::HashMap;
use brg_scene::prelude::{AssetMGADoodad, AssetMGADoodadCategory, SupAssets};

use super::res_state::ResPanelState;

pub fn sys_init_state(assets: SupAssets, mut state: ResMut<ResPanelState>) {
    if state.initialized {
        return;
    }

    state.initialized = true;
    let categories = assets.all::<AssetMGADoodadCategory>();
    let doodads = assets.all::<AssetMGADoodad>();

    // populate from assets
    state.available_assets = HashMap::with_capacity(categories.len());

    for cat in categories {
        state
            .available_assets
            .insert(cat.id.into(), Vec::with_capacity(64));
    }

    for doodad in doodads {
        let list = state.available_assets.get_mut(&doodad.category).unwrap();
        list.push(doodad.id);
    }
}
