use anyhow::Error;
use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetLoader, LoadContext};
use bevy::prelude::TypePath;

use crate::prelude::LevelData;

#[derive(Asset, TypePath)]
pub struct AssetLevel {
    pub level: LevelData,
}

pub struct AssetLevelLoader;

impl AssetLoader for AssetLevelLoader {
    type Asset = AssetLevel;
    type Settings = ();
    type Error = Error;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        Ok(AssetLevel {
            level: LevelData::decode(bytes)?,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["land.bin"]
    }
}
