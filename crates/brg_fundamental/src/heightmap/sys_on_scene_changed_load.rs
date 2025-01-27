use bevy::prelude::{error, info, Assets, Res, ResMut};
use brg_core::prelude::T_LIB_CONT_SIZE_SQ;
use brg_scene::prelude::{AssetLevel, GameAssets};

use super::dto_landscape::{Landscape, LandscapeArea, LandscapeChunk};
use super::res::ResLandscape;

pub fn sys_on_scene_changed_load_level(
    mut res: ResMut<ResLandscape>,
    game: Res<GameAssets>,
    levels: Res<Assets<AssetLevel>>,
) {
    let data = levels.get(&game.level);
    if data.is_none() {
        error!("cant find level asset to load");
        return;
    }

    let data = data.unwrap();

    // copy data to local res
    let (width, height) = (data.level.width(), data.level.height());

    let mut land = Landscape {
        width,
        height,
        areas: Vec::with_capacity((width * height) as usize),
    };

    for area in &data.level.landscape().areas {
        land.areas.push(LandscapeArea {
            heights:    area.heights,
            has_chunks: area.has_chunks,
            chunks:     match area.has_chunks {
                false => vec![],
                true => {
                    let mut chunks = Vec::with_capacity(T_LIB_CONT_SIZE_SQ);
                    for chunk in &area.chunks {
                        chunks.push(LandscapeChunk {
                            heights: chunk.heights,
                        });
                    }

                    chunks
                }
            },
        });
    }

    // set
    res.width = width;
    res.height = height;
    res.landscape = land;

    // notify
    info!(
        "level {} loaded with [{}x{}] areas - successfully!",
        data.level.name(),
        width,
        height
    );
}
