use std::ops::{AddAssign, *};

use bevy::prelude::Reflect;

use crate::components::lib::V2;
use crate::components::tiles::consts::{TILES_PER_CHUNK, TILES_PER_CHUNK_HALF};
use crate::components::tiles::Chunk;

/// Tile is 2D tile (minimum logical unit in grid)
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug, Default, Reflect)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
}

#[allow(dead_code)]
impl Tile {
    pub const SIZE: V2 = V2::ONE;

    #[inline(always)]
    pub fn at(x: i32, y: i32) -> Tile {
        Tile { x, y }
    }

    #[inline(always)]
    pub fn xy(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    /// Tile top-left world position
    #[inline(always)]
    pub fn position(&self) -> V2 {
        V2 {
            x: self.x as f32,
            y: self.y as f32,
        }
    }

    /// Tile center position in absolute world coordinates
    #[inline(always)]
    pub fn position_center(&self) -> V2 {
        return self.position() + 0.5;
    }

    #[inline(always)]
    pub fn size(&self) -> V2 {
        Self::SIZE
    }

    #[inline]
    pub fn chunk(&self) -> Chunk {
        Chunk {
            x: (((self.x as f32) + TILES_PER_CHUNK_HALF) / (TILES_PER_CHUNK as f32)).floor() as i32,
            y: (((self.y as f32) + TILES_PER_CHUNK_HALF) / (TILES_PER_CHUNK as f32)).floor() as i32,
        }
    }
}

pub trait VecExt {
    fn tile(&self) -> Tile;
}

impl VecExt for V2 {
    fn tile(&self) -> Tile {
        Tile {
            x: self.x.floor() as i32,
            y: self.y.floor() as i32,
        }
    }
}

#[auto_impl_ops::auto_ops]
impl AddAssign<&Tile> for Tile {
    fn add_assign(&mut self, other: &Self) {
        self.x = &self.x + &other.x;
        self.y = &self.y + &other.y;
    }
}

#[auto_impl_ops::auto_ops]
impl SubAssign<&Tile> for Tile {
    fn sub_assign(&mut self, other: &Self) {
        self.x = &self.x - &other.x;
        self.y = &self.y - &other.y;
    }
}
