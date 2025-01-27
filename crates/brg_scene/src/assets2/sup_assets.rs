use bevy::asset::Assets;
use bevy::ecs::system::SystemParam;
use bevy::prelude::{error, warn, Res};
use brg_core::prelude::{Id, IdCategory};

use super::assets_mgas::{AssetMGA, MgaTypedData};
use super::res_storage::ResAssetsStorage;

#[derive(SystemParam)]
pub struct SupAssets<'w> {
    storage:    Res<'w, ResAssetsStorage>,
    assets_mga: Res<'w, Assets<AssetMGA>>,
}

impl<'w> SupAssets<'w> {
    pub fn get<T: MgaTypedData>(&self, id: Id) -> Option<T> {
        let Some(h) = self.storage.assets_mga_by_id.get(&id) else {
            error!("Not found MGA asset by id: {}", id);
            return None;
        };

        let Some(table) = self.assets_mga.get(h) else {
            error!("Not found MGA asset by handle: {:?}", h);
            return None;
        };

        let Some(instance) = table.0.get(&id) else {
            error!("Not found MGA instance by id {} in table {:?}", id, h);
            return None;
        };

        Some(instance.get::<T>())
    }

    pub fn all<T: MgaTypedData>(&self, category: IdCategory) -> Vec<T> {
        let len = self.storage.assets_mga_by_category.len();
        let list = self.storage.assets_mga_by_category.get(&category).unwrap();

        let mut result: Vec<T> = Vec::with_capacity(len);

        for id in list {
            if let Some(asset) = self.get::<T>(*id) {
                result.push(asset);
            }
        }

        result
    }
}
