use bevy::prelude::{Added, Commands, Entity, Query, ResMut};
use brg_core::prelude::{Block, Tile, VecExt};

use crate::prelude::{CmpExternalHeightmapDataImporter, CmpTransform2D, ResHeightmap};

pub fn import_heightmap_from_entities(
    query: Query<
        (Entity, &CmpExternalHeightmapDataImporter, &CmpTransform2D),
        Added<CmpExternalHeightmapDataImporter>,
    >,
    mut hm: ResMut<ResHeightmap>,
    mut cmd: Commands,
) {
    for (ent, data, trm) in &query {
        let center_tile = trm.position.tile();
        let min_y = center_tile.y - (data.height / 2) as i32;
        let max_y = center_tile.y + (data.height / 2) as i32;
        let min_x = center_tile.x - (data.width / 2) as i32;
        let max_x = center_tile.x + (data.width / 2) as i32;

        let mut ind = 0;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let height_next = data.points[ind];
                let height_cur = *hm.tiles.get(&Tile::at(x, y)).unwrap_or(&0.0) as f32;

                hm.tiles
                    .insert(Tile::at(x, y), f32::max(height_cur, height_next));
                ind += 1;
            }
        }

        cmd.entity(ent).remove::<CmpExternalHeightmapDataImporter>();
    }
}
