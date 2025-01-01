use bevy::prelude::{ReflectResource, Resource};
use bevy::reflect::Reflect;
use bevy::utils::HashMap;

use crate::prelude::consts::TERRAIN_HEIGHT;
use crate::prelude::types::NormalizedF32;
use crate::prelude::{Tile, VecExt, V2};

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct ResHeightmap {
    pub tiles: HashMap<Tile, NormalizedF32>,
}

impl Default for ResHeightmap {
    fn default() -> Self {
        let mut tiles = HashMap::default();

        // todo: delete this
        tiles.insert(Tile::at(0, 0), 0.2);
        tiles.insert(Tile::at(1, 0), 0.3);
        tiles.insert(Tile::at(2, 0), 0.5);
        tiles.insert(Tile::at(3, 0), 0.6);
        tiles.insert(Tile::at(4, 0), 0.8);
        tiles.insert(Tile::at(0, 1), 0.2);
        tiles.insert(Tile::at(1, 1), 0.3);
        tiles.insert(Tile::at(2, 1), 0.5);
        tiles.insert(Tile::at(3, 1), 0.6);
        tiles.insert(Tile::at(4, 1), 0.8);
        tiles.insert(Tile::at(0, 2), 0.2);
        tiles.insert(Tile::at(1, 2), 0.3);
        tiles.insert(Tile::at(2, 2), 0.5);
        tiles.insert(Tile::at(3, 2), 0.6);
        tiles.insert(Tile::at(4, 2), 0.2);

        Self { tiles }
    }
}

impl ResHeightmap {
    pub fn height_at_pos(&self, pos: V2) -> f32 {
        let (tl, tr, bl, br) = self.heights_of_tile(pos.tile());
        let uv = (pos - pos.tile().position()) / pos.tile().size();
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

    #[inline]
    fn height_at(&self, tile: Tile) -> NormalizedF32 {
        self.tiles.get(&tile).copied().unwrap_or(0.0)
    }

    // heights in order: TL, TR, BL, BR
    #[inline]
    fn heights_of_tile(
        &self,
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
