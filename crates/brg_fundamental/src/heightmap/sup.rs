use bevy::ecs::system::SystemParam;
use bevy::prelude::Res;
use brg_core::prelude::types::NormalizedF32;
use brg_core::prelude::{Block, BlockPosition, Tile, VecExt, T_LIB_TILE_SIZE_SQ, V2};

use super::res::ResLandscape;

#[derive(SystemParam)]
pub struct SupHeightmap<'w> {
    landscape: Res<'w, ResLandscape>,
}

impl<'w> SupHeightmap<'w> {
    pub fn world_size(&self) -> V2 {
        V2::new(self.landscape.width as f32, self.landscape.height as f32)
    }

    fn height_at(&mut self, tile: Tile) -> NormalizedF32 {
        // apply offset (map center should be at 0,0)
        let tile = tile + self.landscape.offset;

        if tile.x < 0 || tile.y < 0 {
            return 0.0;
        }

        if tile.x > self.landscape.width as i32 || tile.y > self.landscape.height as i32 {
            return 0.0;
        }

        let ind = tile.y as usize * self.landscape.width as usize + tile.x as usize;
        if ind >= self.landscape.values.len() {
            return 0.0;
        }

        self.landscape.values[ind]
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

        z * self.landscape.volume as f32
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
