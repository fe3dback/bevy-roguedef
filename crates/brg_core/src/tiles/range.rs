use std::collections::HashSet;

use super::chunk::Chunk;
use super::tile::Tile;
use crate::vectors::prelude::V2;

// Range is box tiles selection with
// any size, that points on specified tiles
// all tiles indexes in this Range is inclusive
// Empty ranges is not possible, because
// range will point at least to one Tile (when min=max)
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct Range {
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
}

pub struct RangeIter<'a> {
    data:   &'a Range,
    cursor: Tile,
}

impl<'a> IntoIterator for &'a Range {
    type Item = Tile;
    type IntoIter = RangeIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        RangeIter {
            data:   &self,
            cursor: Tile::at(self.min_x - 1, self.min_y),
        }
    }
}

impl Iterator for RangeIter<'_> {
    type Item = Tile;
    fn next(&mut self) -> Option<Self::Item> {
        // left -> right
        self.cursor.x += 1;

        // top -> bottom
        if self.cursor.x > self.data.max_x {
            self.cursor.x = self.data.min_x;
            self.cursor.y += 1;
        }

        if self.cursor.y > self.data.max_y {
            return None;
        }

        return Some(self.cursor);
    }
}

#[allow(dead_code)]
impl Range {
    #[inline]
    pub fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Range {
        Range {
            min_x: min_x.min(max_x),
            min_y: min_y.min(max_y),
            max_x: min_x.max(max_x),
            max_y: min_y.max(max_y),
        }
    }

    #[inline]
    pub fn width(&self) -> i32 {
        self.max_x - self.min_x + 1
    }

    #[inline]
    pub fn height(&self) -> i32 {
        self.max_y - self.min_y + 1
    }

    #[inline]
    pub fn len(&self) -> usize {
        (self.width() * self.height()) as usize
    }

    /// Chunk top-left position in absolute world coordinates
    /// alias for `min.position_px()`
    #[inline]
    pub fn position(&self) -> V2 {
        Tile::at(self.min_x, self.min_y).position()
    }

    /// Chunk center position in absolute world coordinates
    pub fn position_center(&self) -> V2 {
        let tl = self.position();
        let size = self.size();
        V2 {
            x: tl.x + (size.x / 2.0),
            y: tl.y + (size.y / 2.0),
        }
    }

    #[inline]
    pub fn size(&self) -> V2 {
        V2 {
            x: self.width() as f32,
            y: self.height() as f32,
        }
    }

    #[inline]
    pub fn has_tile(&self, tile: Tile) -> bool {
        (self.min_x <= tile.x && tile.x <= self.max_x)
            && (self.min_y <= tile.y && tile.y <= self.max_y)
    }

    #[inline]
    // check if range intersected with another range at least with single tile
    pub fn intersected_with(&self, another: Range) -> bool {
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

        return true;
    }

    #[inline]
    // check if range contains at least one tile of chunk
    pub fn has_any_tile_of_chunk(&self, chunk: Chunk) -> bool {
        return self.intersected_with(chunk.child_tiles_range());
    }

    pub fn chunks(&self) -> Vec<Chunk> {
        // todo: optimize alg with math
        let mut list: Vec<Chunk> = Vec::new();
        let mut handled: HashSet<(i32, i32)> = HashSet::new();

        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                let chunk = Tile::at(x, y).chunk();
                let chunk_id = (chunk.x, chunk.y);
                if handled.contains(&chunk_id) {
                    continue;
                }

                list.push(chunk);
                handled.insert(chunk_id);
            }
        }

        list
    }
}
