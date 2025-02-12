use anyhow::{bail, Error};
use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetLoader, LoadContext};
use bevy::prelude::TypePath;
use binary_rw::{BinaryReader, Endian, MemoryStream};

#[derive(Asset, TypePath)]
pub struct AssetLevel {
    pub level:  Vec<f32>,
    pub width:  u32,
    pub height: u32,
    pub name:   String,
}

#[derive(Debug)]
enum Quality {
    R8,
    R16,
    R32,
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

        let quality = match ctx.asset_path().get_full_extension() {
            Some(ext) => match ext.as_str() {
                "heightmap.r8" => Quality::R8,
                "heightmap.r16" => Quality::R16,
                "heightmap.r32" => Quality::R32,
                _ => bail!("unsupported file format"),
            },
            None => bail!("unsupported file format"),
        };

        let width = match quality {
            Quality::R8 => bytes.len().isqrt() as u32,
            Quality::R16 => (bytes.len() / 2).isqrt() as u32,
            Quality::R32 => (bytes.len() / 4).isqrt() as u32,
        };
        let height = width;
        let name = ctx
            .path()
            .parent()
            .map(|path| path.to_string_lossy().to_string())
            .unwrap_or(String::new());

        let mut data = MemoryStream::from(bytes);
        let mut reader = BinaryReader::new(&mut data, Endian::Little);
        let mut height_values: Vec<f32> = Vec::with_capacity((width * height) as usize);

        for _ in 0..height {
            for _ in 0..width {
                let height: f32 = match quality {
                    Quality::R8 => (reader.read_u8()? as f32) / u8::MAX as f32,
                    Quality::R16 => (reader.read_u16()? as f32) / u16::MAX as f32,
                    Quality::R32 => reader.read_f32()?,
                };

                height_values.push(height);
            }
        }

        Ok(AssetLevel {
            level: height_values,
            width,
            height,
            name,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["heightmap.r8", "heightmap.r16", "heightmap.r32"]
    }
}
