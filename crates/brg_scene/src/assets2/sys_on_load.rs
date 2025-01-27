use bevy::prelude::{Assets, EventReader, Res, ResMut};

use super::assets_mgas::AssetMGA;
use super::evt_on_load::EvtOnLoad;
use super::res_loading_state::ResLoadingState;
use super::res_storage::ResAssetsStorage;

pub fn sys_on_load(
    mut reader: EventReader<EvtOnLoad>,
    mut storage: ResMut<ResAssetsStorage>,
    mut loading_state: ResMut<ResLoadingState>,
    assets_mga: Res<Assets<AssetMGA>>,
) {
    for ev in reader.read() {
        // cache MGA assetID -> handle
        {
            if let Ok(handle_mga) = ev.handle.clone().try_typed::<AssetMGA>() {
                let asset = assets_mga.get(&handle_mga).unwrap();
                asset.0.keys().for_each(|id| {
                    storage.assets_mga_by_id.insert(*id, handle_mga.clone());

                    let by_category_list = storage.assets_mga_by_category.get_mut(&id.category());
                    by_category_list.unwrap().push(*id);
                });
            };
        }

        loading_state.status.cnt_ready += 1;
    }
}
