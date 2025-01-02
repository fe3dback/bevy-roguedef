mod asset;
mod asset_creatures;
mod asset_hm_data;
pub mod plug;

pub mod prelude {
    pub use super::asset::GameAssets;
    pub use super::asset_creatures::*;
    pub use super::asset_hm_data::*;
}
