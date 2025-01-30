use bevy::prelude::{Handle, Image, Resource};
use bevy::utils::hashbrown::HashMap;
use brg_core::prelude::{Id, IdCategory};
use strum::{EnumCount, IntoEnumIterator};

use super::asset_level::AssetLevel;
use super::assets_mgas::AssetMGA;

#[derive(Resource)]
pub struct ResAssetsStorage {
    pub(super) assets_mga_by_id:       HashMap<Id, Handle<AssetMGA>>,
    pub(super) assets_mga_by_category: HashMap<IdCategory, Vec<Id>>,

    pub landscape: Landscape,
}

#[derive(Default)]
pub struct Landscape {
    pub level:                Handle<AssetLevel>,
    pub texture_world_albedo: Handle<Image>,
    pub texture_ground_grass: Handle<Image>,
}

impl Default for ResAssetsStorage {
    fn default() -> Self {
        let mut mga_by_category: HashMap<IdCategory, Vec<Id>> =
            HashMap::with_capacity(IdCategory::COUNT);

        for cat in IdCategory::iter() {
            mga_by_category.insert(cat, Vec::with_capacity(128));
        }

        Self {
            landscape:              Landscape::default(),
            assets_mga_by_id:       HashMap::with_capacity(512),
            assets_mga_by_category: mga_by_category,
        }
    }
}
