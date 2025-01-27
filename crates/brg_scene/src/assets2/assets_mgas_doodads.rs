use brg_core::prelude::Id;
use serde::Deserialize;

use super::assets_mgas::{AssetMGAInstance, MgaTypedData};

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct AssetMGADoodad {
    pub id:       Id,
    #[serde(rename = "cat")]
    pub category: Id,
}

impl MgaTypedData for AssetMGADoodad {
    #[inline]
    fn get(data: &AssetMGAInstance) -> &Self {
        match &data.doodad {
            Some(v) => v,
            None => panic!("MGA Is not doodad"),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AssetMGADoodadCategory {
    pub id:    Id,
    pub title: String,
}

impl MgaTypedData for AssetMGADoodadCategory {
    #[inline]
    fn get(data: &AssetMGAInstance) -> &Self {
        match &data.doodad_category {
            Some(v) => v,
            None => panic!("MGA Is not doodad_category"),
        }
    }
}
