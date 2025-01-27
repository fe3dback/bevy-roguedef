use bevy::prelude::{Asset, TypePath};
use bevy::utils::hashbrown::HashMap;
use brg_core::prelude::{Id, IdCategory};

use super::assets_mgas_doodads::{AssetMGADoodad, AssetMGADoodadCategory};

pub trait MgaTypedData {
    fn get(data: &AssetMGAInstance) -> &Self;
    fn category() -> IdCategory;
}

#[derive(Asset, TypePath)]
pub struct AssetMGA(pub HashMap<Id, AssetMGAInstance>);

#[derive(Default)]
pub struct AssetMGAInstance {
    pub doodad:          Option<AssetMGADoodad>,
    pub doodad_category: Option<AssetMGADoodadCategory>,
    pub unit:            Option<bool>, // todo
}

impl AssetMGAInstance {
    #[inline]
    pub fn get<T: MgaTypedData>(&self) -> &T {
        T::get(self)
    }
}

pub mod loader {
    use anyhow::{anyhow, bail, Context, Error};
    use bevy::asset::io::Reader;
    use bevy::asset::{AssetLoader, LoadContext};
    use bevy::prelude::default;
    use bevy::utils::hashbrown::HashMap;
    use brg_core::prelude::{Id, IdCategory};
    use serde::Deserialize;

    use crate::assets2::assets_mgas::{AssetMGA, AssetMGAInstance};

    #[derive(Deserialize)]
    struct RawMGA {
        id_asset:  RawMGAId,
        raw_bytes: Vec<u8>,
    }

    #[derive(Deserialize)]
    struct RawMGAId {
        id: Id,
    }

    #[derive(Default)]
    pub struct AssetMGALoader {}

    impl AssetLoader for AssetMGALoader {
        type Asset = AssetMGA;
        type Settings = ();
        type Error = String;

        async fn load(
            &self,
            reader: &mut dyn Reader,
            _settings: &Self::Settings,
            _: &mut LoadContext<'_>,
        ) -> Result<Self::Asset, Self::Error> {
            match load(reader).await {
                Ok(asset) => Ok(asset),
                Err(err) => Err(format!("{:#}", err)),
            }
        }

        fn extensions(&self) -> &[&str] {
            &["mga.ron"]
        }
    }

    async fn load(reader: &mut dyn Reader) -> Result<AssetMGA, Error> {
        let mut bytes = Vec::with_capacity(128);
        reader
            .read_to_end(&mut bytes)
            .await
            .context("cannot read bytes")?;

        // load raw data
        let list =
            split_bytes_into_assets(bytes).context("failed split file into MGA assets list")?;

        // ensure that all assets in file has same category
        ensure_category_is_same(&list)
            .context("check that every asset in list have same category")?;

        // decode every asset into own type
        let mut file_content: HashMap<Id, AssetMGAInstance> = HashMap::with_capacity(list.len());
        for raw_asset in list {
            let id = raw_asset.id_asset.id;
            file_content.insert(
                raw_asset.id_asset.id,
                decode_generic_mga(raw_asset).context(format!("failed decode MGA '{}'", id))?,
            );
        }

        Ok(AssetMGA(file_content))
    }

    fn split_bytes_into_assets(bytes: Vec<u8>) -> Result<Vec<RawMGA>, Error> {
        let mut result: Vec<RawMGA> = Vec::with_capacity(16);
        let mut buff: Vec<u8> = Vec::with_capacity(256);
        let mut balance = 0;
        let mut skip_next_delimiter = false;

        // 1..-2 - that remove '[' ...content... ']' required array bracers
        for byte in &bytes[1..bytes.len() - 2] {
            if *byte == b',' && skip_next_delimiter {
                skip_next_delimiter = false;
                continue;
            }

            buff.push(*byte);

            match byte {
                b'(' => {
                    balance += 1;
                }
                b')' => {
                    balance -= 1;
                    if balance == 0 {
                        let bytes = buff.clone();
                        let id_asset: RawMGAId =
                            ron::from_str(&String::from_utf8(bytes.clone()).context("not utf8")?)
                                .context("decode ron bytes")?;

                        result.push(RawMGA {
                            id_asset,
                            raw_bytes: bytes,
                        });
                        buff.clear();
                        skip_next_delimiter = true
                    }
                }
                _ => {}
            }
        }

        Ok(result)
    }

    fn ensure_category_is_same(list: &Vec<RawMGA>) -> Result<(), Error> {
        let mut common_category: Option<IdCategory> = None;

        for asset in list {
            let my_category = asset.id_asset.id.category();

            if common_category.is_none() {
                common_category = Some(my_category);
                continue;
            }

            let another_category = common_category.unwrap();

            if another_category != my_category {
                bail!(
                    "assets is file have different categories ({} vs {}) in '{}'",
                    my_category,
                    another_category,
                    asset.id_asset.id,
                );
            }
        }

        Ok(())
    }

    fn decode_generic_mga(asset: RawMGA) -> Result<AssetMGAInstance, Error> {
        match asset.id_asset.id.category() {
            IdCategory::Doodads => Ok(AssetMGAInstance {
                doodad: Some(ron::from_str(&String::from_utf8(asset.raw_bytes)?)?),
                ..default()
            }),
            IdCategory::DoodadsCategory => Ok(AssetMGAInstance {
                doodad_category: Some(ron::from_str(&String::from_utf8(asset.raw_bytes)?)?),
                ..default()
            }),
            IdCategory::Units => Ok(AssetMGAInstance {
                unit: Some(true), // todo
                ..default()
            }),
            IdCategory::Unknown => Err(anyhow!("asset category is 'Unknown'")),
        }
    }
}
