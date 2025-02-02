use anyhow::Error;
use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetLoader, LoadContext};
use bevy::prelude::TypePath;

#[derive(Asset, TypePath)]
pub struct AssetLevel {
    pub level:  Vec<u8>,
    pub width:  u32,
    pub height: u32,
    pub name:   String,
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
        ctx: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let width = bytes.len().isqrt() as u32;
        let height = width;
        let name = ctx
            .path()
            .parent()
            .map(|path| path.to_string_lossy().to_string())
            .unwrap_or(String::new());

        Ok(AssetLevel {
            level: bytes,
            width,
            height,
            name,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["heightmap.r8"]
    }
}
