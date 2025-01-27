use brg_core::prelude::{ICDoodads, ICDoodadsCategory, IdCategory, IdOf};
use serde::Deserialize;

use super::assets_mgas::{AssetMGAInstance, MgaTypedData};

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct AssetMGADoodad {
    pub id:       IdOf<ICDoodads>,
    #[serde(rename = "cat")]
    pub category: IdOf<ICDoodadsCategory>,
}

impl MgaTypedData for AssetMGADoodad {
    #[inline]
    fn get(data: &AssetMGAInstance) -> &Self {
        match &data.doodad {
            Some(v) => v,
            None => panic!("MGA Is not doodad"),
        }
    }

    fn category() -> IdCategory {
        IdCategory::Doodads
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AssetMGADoodadCategory {
    pub id:    IdOf<ICDoodadsCategory>,
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

    fn category() -> IdCategory {
        IdCategory::DoodadsCategory
    }
}
