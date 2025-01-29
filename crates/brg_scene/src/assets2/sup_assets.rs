use bevy::asset::Assets;
use bevy::ecs::system::SystemParam;
use bevy::prelude::{error, Handle, Res, ResMut};
use brg_core::prelude::Id;

use super::assets_mgas::{AssetMGA, MgaTypedData};
use super::res_storage::ResAssetsStorage;
use super::asset_level::AssetLevel;

#[derive(SystemParam)]
pub struct SupAssets<'w> {
    pub(super) storage:    ResMut<'w, ResAssetsStorage>,
    pub(super) assets_mga: Res<'w, Assets<AssetMGA>>,
}

impl<'w> SupAssets<'w> {
    pub fn level(&self) -> Handle<AssetLevel> {
        self.storage.level.clone()
    }

    pub fn get<T: MgaTypedData, R: AsRef<Id>>(&self, id: R) -> Option<&T> {
        let id = id.as_ref();

        let Some(h) = self.storage.assets_mga_by_id.get(id) else {
            error!("Not found MGA asset by id: {}", id);
            return None;
        };

        let Some(table) = self.assets_mga.get(h) else {
            error!("Not found MGA asset by handle: {:?}", h);
            return None;
        };

        let Some(instance) = table.0.get(id) else {
            error!("Not found MGA instance by id {} in table {:?}", id, h);
            return None;
        };

        Some(instance.get::<T>())
    }

    pub fn has(&self, id: Id) -> bool {
        let Some(h) = self.storage.assets_mga_by_id.get(&id) else {
            return false;
        };

        let Some(table) = self.assets_mga.get(h) else {
            return false;
        };

        let Some(_) = table.0.get(&id) else {
            return false;
        };

        true
    }

    pub fn all<T: MgaTypedData>(&self) -> Vec<&T> {
        let len = self.storage.assets_mga_by_category.len();
        let list = self
            .storage
            .assets_mga_by_category
            .get(&T::category())
            .unwrap();

        let mut result: Vec<&T> = Vec::with_capacity(len);

        for id in list {
            if let Some(asset) = self.get::<T, _>(id) {
                result.push(asset);
            }
        }

        result
    }
}
