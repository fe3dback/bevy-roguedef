use super::block::{Area, Chunk, Cluster, Tile};
use super::consts::ELEMENTS_IN_CONTAINER;
use crate::prelude::{TILE_SIZE, TILE_SIZE_SQ, V2};

#[allow(dead_code)]
pub trait BlockSize {
    fn size(&self) -> (i32, i32);
    fn size_m(&self) -> V2;
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

impl Tile {
    /// return 1
    #[inline(always)]
    pub const fn size() -> i32 {
        1
    }

    /// return V2::ONE (1, 1)
    #[inline(always)]
    pub const fn size_m() -> V2 {
        TILE_SIZE_SQ
    }
}

impl Chunk {
    /// return 15 = container (size=width=height) of tiles in container
    #[inline(always)]
    pub const fn size() -> i32 {
        ELEMENTS_IN_CONTAINER
    }

    /// return 15m
    #[inline(always)]
    pub const fn size_m() -> V2 {
        V2::new(
            TILE_SIZE * ELEMENTS_IN_CONTAINER as f32,
            TILE_SIZE * ELEMENTS_IN_CONTAINER as f32,
        )
    }
}

impl Area {
    /// return 15 = container (size=width=height) of chunks in container
    #[inline(always)]
    pub const fn size() -> i32 {
        ELEMENTS_IN_CONTAINER
    }

    /// return 225m
    #[inline(always)]
    pub const fn size_m() -> V2 {
        V2::new(
            Chunk::size_m().x * ELEMENTS_IN_CONTAINER as f32,
            Chunk::size_m().y * ELEMENTS_IN_CONTAINER as f32,
        )
    }
}

impl Cluster {
    /// return 15 = container (size=width=height) of areas in container
    #[inline(always)]
    pub const fn size() -> i32 {
        ELEMENTS_IN_CONTAINER
    }

    /// return 3_375m
    #[inline(always)]
    pub const fn size_m() -> V2 {
        V2::new(
            Area::size_m().x * ELEMENTS_IN_CONTAINER as f32,
            Area::size_m().y * ELEMENTS_IN_CONTAINER as f32,
        )
    }
}
