use bevy::prelude::{Handle, Resource};
use bevy::utils::hashbrown::HashMap;
use brg_core::prelude::Id;

use super::assets_mgas::AssetMGA;

#[derive(Default, Eq, PartialEq, Clone, Copy, Debug)]
pub enum ELoadingStage {
    #[default]
    CalculateAssetsToLoad,
    Loading,
    Ready,
}

#[derive(Resource, Default)]
pub struct ResAssetsStorage {
    pub assets_mga:       HashMap<Handle<AssetMGA>, AssetMGA>,
    pub assets_mga_by_id: HashMap<Id, Handle<AssetMGA>>,
}
