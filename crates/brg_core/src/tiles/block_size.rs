use super::block::{Area, Chunk, Cluster, Tile};
use super::consts::T_LIB_CONT_ROW_LEN;
use crate::prelude::{T_LIB_TILE_SIZE, T_LIB_TILE_SIZE_SQ, V2};

impl Tile {
    /// return 1
    #[inline(always)]
    pub const fn size() -> usize {
        1
    }

    /// return V2::ONE (1, 1)
    #[inline(always)]
    pub const fn size_m() -> V2 {
        T_LIB_TILE_SIZE_SQ
    }
}

impl Chunk {
    /// return 16 = container (size=width=height) of tiles in container
    #[inline(always)]
    pub const fn size() -> usize {
        T_LIB_CONT_ROW_LEN
    }

    /// return 16m
    #[inline(always)]
    pub const fn size_m() -> V2 {
        V2::new(
            T_LIB_TILE_SIZE * T_LIB_CONT_ROW_LEN as f32,
            T_LIB_TILE_SIZE * T_LIB_CONT_ROW_LEN as f32,
        )
    }
}

impl Area {
    /// return 16 = container (size=width=height) of chunks in container
    #[inline(always)]
    pub const fn size() -> usize {
        T_LIB_CONT_ROW_LEN
    }

    /// return 256m
    #[inline(always)]
    pub const fn size_m() -> V2 {
        V2::new(
            Chunk::size_m().x * T_LIB_CONT_ROW_LEN as f32,
            Chunk::size_m().y * T_LIB_CONT_ROW_LEN as f32,
        )
    }
}

impl Cluster {
    /// return 16 = container (size=width=height) of areas in container
    #[inline(always)]
    pub const fn size() -> usize {
        T_LIB_CONT_ROW_LEN
    }

    /// return 4096m
    #[inline(always)]
    pub const fn size_m() -> V2 {
        V2::new(
            Area::size_m().x * T_LIB_CONT_ROW_LEN as f32,
            Area::size_m().y * T_LIB_CONT_ROW_LEN as f32,
        )
    }
}
