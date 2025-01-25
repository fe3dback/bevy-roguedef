mod asset;
mod asset_creatures;
mod asset_doodad;
mod asset_effect;
mod asset_level;
mod asset_projectile;
mod asset_sound;
mod asset_spell;
mod asset_weapon;
pub mod plug;

pub mod prelude {
    pub use super::asset::GameAssets;
    pub use super::asset_creatures::*;
    pub use super::asset_effect::*;
    pub use super::asset_level::AssetLevel;
    pub use super::asset_projectile::*;
    pub use super::asset_sound::*;
    pub use super::asset_spell::*;
    pub use super::asset_weapon::*;
}
