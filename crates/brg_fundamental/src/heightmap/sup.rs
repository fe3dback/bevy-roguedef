use std::ops::Deref;

use bevy::ecs::system::SystemParam;
use bevy::prelude::{Res, ResMut};
use brg_core::prelude::consts::TERRAIN_HEIGHT;
use brg_core::prelude::types::NormalizedF32;
use brg_core::prelude::{
    Area,
    Block,
    BlockChild,
    BlockParent,
    BlockPosition,
    Chunk,
    Range,
    Tile,
    VecExt,
    T_LIB_CONT_SIZE_SQ,
    T_LIB_TILE_SIZE_SQ,
    V2,
};

use super::res::{ResHeightmapCache, ResLandscape};

#[derive(SystemParam)]
pub struct SupHeightmap<'w> {
    cache:     ResMut<'w, ResHeightmapCache>,
    landscape: Res<'w, ResLandscape>,
}

impl<'w> SupHeightmap<'w> {
    pub fn world_size(&self) -> V2 {
        V2::new(
            self.landscape.width as f32 * Area::size_m().x,
            self.landscape.height as f32 * Area::size_m().y,
        )
    }

    fn height_at(&mut self, tile: Tile) -> NormalizedF32 {
        let h = self.cache.tiles.get(&tile);

        match h {
            Some(v) => *v,
            None => {
                self.load_heights_from_landscape(tile);
                self.cache.tiles.get(&tile).copied().unwrap_or(0.0)
            }
        }
    }

    fn load_heights_from_landscape(&mut self, tile: Tile) {
        let chunk = tile.parent();

        let min = chunk - Chunk::at(1, 1);
        let max = chunk + Chunk::at(1, 1);
        let neighbors: Range<Chunk> = Range::new(min.x, min.y, max.x, max.y);

        for neighbor in &neighbors {
            if self.cache.loaded_chunks.contains(&neighbor) {
                continue;
            }

            if let Some(heights) = self.landscape.landscape.chunk_heights(neighbor) {
                self.upload_heights(neighbor, *heights.deref());
            }
        }
    }

    fn upload_heights(&mut self, chunk: Chunk, heights: [f32; T_LIB_CONT_SIZE_SQ]) {
        // free if needed
        {
            if self.cache.queue.len() == self.cache.queue.capacity() {
                let chunk_to_drop = self.cache.queue.pop_front().unwrap();
                self.cache.loaded_chunks.remove(&chunk_to_drop);
                for tile in &chunk_to_drop.child_range() {
                    self.cache.tiles.remove(&tile);
                }
            }
        }

        // cache new tiles
        {
            for (ind, tile) in chunk.child_range().into_iter().enumerate() {
                let h = heights[ind];
                self.cache.tiles.insert(tile, h);
            }

            self.cache.queue.push_back(chunk);
            self.cache.loaded_chunks.insert(chunk);
        }
    }

    pub fn height_at_pos(&mut self, pos: V2) -> f32 {
        let (tl, tr, bl, br) = self.heights_of_tile(pos.tile());
        let uv = (pos - pos.tile().position_tl()) / T_LIB_TILE_SIZE_SQ;
        let (u, v) = (uv.x, uv.y);

        // 1            4
        // # ---------- #
        // |    .       |
        // |       .    |
        // # ---------- #
        // 2            3

        // TriangleA = <1,2,3>
        // TriangleB = <3,4,1>

        let z = match u > v {
            true => tl + (u * (tr - tl)) + (v * (br - tr)),
            false => tl + (u * (br - bl)) + (v * (bl - tl)),
        };

        z * TERRAIN_HEIGHT
    }

    // heights in order: TL, TR, BL, BR
    #[inline]
    fn heights_of_tile(
        &mut self,
        tile: Tile,
    ) -> (NormalizedF32, NormalizedF32, NormalizedF32, NormalizedF32) {
        (
            self.height_at(tile + Tile::at(0, 0)),
            self.height_at(tile + Tile::at(1, 0)),
            self.height_at(tile + Tile::at(0, 1)),
            self.height_at(tile + Tile::at(1, 1)),
        )
    }
}
