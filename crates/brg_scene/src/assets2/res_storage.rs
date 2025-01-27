use bevy::prelude::{Handle, Resource};
use bevy::utils::hashbrown::HashMap;
use brg_core::prelude::{Id, IdCategory};
use strum::{EnumCount, IntoEnumIterator};

use super::assets_mgas::AssetMGA;

#[derive(Resource)]
pub struct ResAssetsStorage {
    pub assets_mga_by_id:       HashMap<Id, Handle<AssetMGA>>,
    pub assets_mga_by_category: HashMap<IdCategory, Vec<Id>>,
}

impl Default for ResAssetsStorage {
    fn default() -> Self {
        let mut mga_by_category: HashMap<IdCategory, Vec<Id>> =
            HashMap::with_capacity(IdCategory::COUNT);

        for cat in IdCategory::iter() {
            mga_by_category.insert(cat, Vec::with_capacity(128));
        }

        Self {
            assets_mga_by_id:       HashMap::with_capacity(512),
            assets_mga_by_category: mga_by_category,
        }
    }
}
