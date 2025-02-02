use bevy::prelude::{error, info, Assets, Res, ResMut};
use brg_core::prelude::consts::TERRAIN_HEIGHT;
use brg_core::prelude::{Block, Tile};
use brg_scene::prelude::{AssetLevel, SupAssets};

use super::res::ResLandscape;

pub fn sys_on_scene_changed_load_level(
    mut res: ResMut<ResLandscape>,
    assets: SupAssets,
    levels: Res<Assets<AssetLevel>>,
) {
    let data = levels.get(&assets.landscape().level);
    if data.is_none() {
        error!("cant find level asset to load");
        return;
    }

    let data = data.unwrap();

    res.width = data.width;
    res.height = data.height;
    res.volume = TERRAIN_HEIGHT as u32;
    res.offset = Tile::at(res.width as i32 / 2, res.height as i32 / 2);
    res.values = data.level.clone();

    // notify
    info!(
        "level loaded with [{}x{}] meters - successfully!",
        res.width, res.height
    );
}
