use std::marker::PhantomData;

use super::block::{Block, Tile};
use super::block_position::BlockPosition;
use super::block_size::BlockSize;
use crate::prelude::V2;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Range<T> {
    pub(super) min_x: i32,
    pub(super) min_y: i32,
    pub(super) max_x: i32,
    pub(super) max_y: i32,
    phantom:          PhantomData<T>,
}

impl<T> Range<T> {
    #[inline(always)]
    pub fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Self {
        Self {
            min_x,
            min_y,
            max_x,
            max_y,
            phantom: Default::default(),
        }
    }

    /// return width in ELEMENTS (tiles, chunks, etc...)
    #[inline]
    pub fn width(&self) -> i32 {
        self.max_x - self.min_x + 1
    }

    /// return height in ELEMENTS (tiles, chunks, etc...)
    #[inline]
    pub fn height(&self) -> i32 {
        self.max_y - self.min_y + 1
    }

    /// return len(width*height) in ELEMENTS (tiles, chunks, etc...)
    #[inline]
    pub fn len(&self) -> usize {
        (self.width() * self.height()) as usize
    }
}

impl BlockSize for Range<Tile> {
    #[inline]
    fn size(&self) -> (i32, i32) {
        (self.width(), self.height())
    }

    /// return range size in meters
    #[inline]
    fn size_m(&self) -> V2 {
        V2 {
            x: self.width() as f32,
            y: self.height() as f32,
        }
    }
}

impl BlockPosition for Range<Tile> {
    /// Range top-left position in absolute world coordinates
    #[inline(always)]
    fn position(&self) -> V2 {
        Tile::at(self.min_x, self.min_y).position()
    }

    /// Range center position in absolute world coordinates
    #[inline(always)]
    fn position_center(&self) -> V2 {
        let tl = self.position();
        let size = self.size_m();
        V2 {
            x: tl.x + (size.x / 2.0),
            y: tl.y + (size.y / 2.0),
        }
    }
}

impl Range<Tile> {
    #[inline]
    pub fn has_tile(&self, tile: Tile) -> bool {
        (self.min_x <= tile.x && tile.x <= self.max_x)
            && (self.min_y <= tile.y && tile.y <= self.max_y)
    }

    #[inline]
    // check if range intersected with another range at least with single tile
    pub fn intersected_with(&self, another: Range<Tile>) -> bool {
        if self.max_x < another.min_x {
            return false;
        }
        if self.min_x > another.max_x {
            return false;
        }
        if self.max_y < another.min_y {
            return false;
        }
        if self.min_y > another.max_y {
            return false;
        }

        true
    }
}
