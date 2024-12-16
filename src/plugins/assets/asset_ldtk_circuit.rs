use anyhow::{anyhow, Error};
use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetLoader, AsyncReadExt, LoadContext};
use bevy::prelude::{info, TypePath};
use bevy::utils::ConditionalSendFuture;
use serde::{Deserialize, Serialize};

use crate::components::tiles::Tile;

#[derive(Asset, Debug, TypePath)]
pub struct AssetLdtkCircuit {
    pub id:           String,
    pub width:        i32,
    pub height:       i32,
    pub px_per_meter: i32,
    pub buildings:    Vec<LdtkCircuitBuilding>,
}

#[derive(Debug)]
pub struct LdtkCircuitBuilding {
    pub name:  String,
    pub pos:   Tile,
    pub class: String,
}

#[derive(Serialize, Deserialize)]
struct LdtkDocument {
    #[serde(alias = "identifier")]
    pub identifier:      String,
    #[serde(alias = "pxWid")]
    pub width_px:        i64,
    #[serde(alias = "pxHei")]
    pub height_px:       i64,
    #[serde(alias = "layerInstances")]
    pub layer_instances: Vec<LayerInstance>,
}

#[derive(Serialize, Deserialize)]
struct LayerInstance {
    #[serde(alias = "__identifier")]
    pub layer_type: String,
    #[serde(alias = "entityInstances")]
    pub entities:   Vec<EntityInstance>,
}

#[derive(Serialize, Deserialize)]
struct EntityInstance {
    #[serde(alias = "__identifier")]
    pub name:     String,
    #[serde(alias = "__grid")]
    pub position: Vec<i32>,
    #[serde(alias = "fieldInstances")]
    pub fields:   Vec<EntityField>,
}

#[derive(Serialize, Deserialize)]
struct EntityField {
    #[serde(alias = "__identifier")]
    pub name:  String,
    #[serde(alias = "__value")]
    pub value: String,
}

// --------------------

#[derive(Default)]
pub struct AssetLdtkCircuitLoader;

impl AssetLoader for AssetLdtkCircuitLoader {
    type Asset = AssetLdtkCircuit;
    type Settings = ();
    type Error = Error;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut buff = String::with_capacity(512);
        reader.read_to_string(&mut buff).await?;

        let asset = decode(buff).expect("unable to decode ldtk file");
        info!("{:?}", asset);
        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        &["ldtkl"]
    }
}

fn decode(buff: String) -> anyhow::Result<AssetLdtkCircuit> {
    let data: LdtkDocument = serde_json::from_str(buff.as_str())?;

    let mut buildings = Vec::with_capacity(data.layer_instances.len());
    for layer in data.layer_instances {
        if layer.layer_type != "ENTITIES" {
            continue;
        }

        for instance in layer.entities {
            if instance.position.len() != 2 {
                return Err(anyhow!(
                    "unexpected entity {} pos {:?}",
                    instance.name,
                    instance.position
                ));
            }

            buildings.push(LdtkCircuitBuilding {
                name:  instance.name,
                pos:   Tile::at(instance.position[0], instance.position[1]),
                class: extract_class_from_fields(instance.fields)?,
            })
        }
    }

    Ok(AssetLdtkCircuit {
        id: data.identifier,
        width: data.width_px as i32,
        height: data.height_px as i32,
        px_per_meter: 20,
        buildings,
    })
}

fn extract_class_from_fields(fields: Vec<EntityField>) -> anyhow::Result<String> {
    for field in fields {
        if field.name != "CLASS" {
            continue;
        }

        return Ok(field.value);
    }

    return Err(anyhow!("not found 'class' field in entity"));
}
