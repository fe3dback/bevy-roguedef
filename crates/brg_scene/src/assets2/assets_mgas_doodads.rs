use brg_core::prelude::Id;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct AssetMGADoodad {
    pub id:       Id,
    #[serde(rename = "cat")]
    pub category: Id,
}
