mod assets_mgas;
mod assets_mgas_doodads;
mod cmp;
mod dto_status;
mod evt_on_load;
pub mod plug;
mod res_loading_state;
mod res_storage;
mod sup_assets;
mod sup_loader;
mod sys_loadscreen;
mod sys_on_load;

pub mod prelude {
    pub use super::assets_mgas_doodads::*;
    pub use super::sup_assets::SupAssets;
}
