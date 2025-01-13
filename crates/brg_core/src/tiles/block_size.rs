use super::block::{Area, Chunk, Cluster, Tile};
use super::consts::ELEMENTS_IN_CONTAINER;
use crate::prelude::{TILE_SIZE_SQ, V2};

#[allow(dead_code)]
pub trait BlockSize {
    fn size(&self) -> (i32, i32);
    fn size_m(&self) -> V2;
}

impl Tile {
    pub const fn size_m() -> V2 {
        TILE_SIZE_SQ
    }
}

impl BlockSize for Tile {
    /// one tile is always one element
    #[inline(always)]
    fn size(&self) -> (i32, i32) {
        (1, 1)
    }

    /// return tile size in meters
    #[inline(always)]
    fn size_m(&self) -> V2 {
        TILE_SIZE_SQ
    }
}

impl Chunk {
    /// return 15 = container (size=width=height) of tiles in container
    #[inline(always)]
    pub const fn size() -> i32 {
        ELEMENTS_IN_CONTAINER
    }
}

impl Area {
    /// return 15 = container (size=width=height) of chunks in container
    #[inline(always)]
    pub const fn size() -> i32 {
        ELEMENTS_IN_CONTAINER
    }
}

impl Cluster {
    /// return 15 = container (size=width=height) of cluster in container
    #[inline(always)]
    pub const fn size() -> i32 {
        ELEMENTS_IN_CONTAINER
    }
}
