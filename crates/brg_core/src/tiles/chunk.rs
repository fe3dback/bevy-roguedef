use std::ops::{AddAssign, SubAssign, *};

use super::area::Area;
use super::consts::{CHUNKS_PER_AREA, CHUNKS_PER_AREA_HALF, TILES_AROUND_CENTER, TILES_PER_CHUNK};
use super::range::Range;
use super::tile::Tile;

// Chunk is 2D array of tiles
// Chunk contain exactly 15x15 tiles (225 total)
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Chunk {
    pub x: i32,
    pub y: i32,
}

impl Default for Chunk {
    fn default() -> Self {
        Self::at(0, 0)
    }
}

#[allow(dead_code)]
impl Chunk {
    #[inline]
    pub const fn size() -> i32 {
        TILES_PER_CHUNK
    }

    #[inline]
    pub fn at(x: i32, y: i32) -> Chunk {
        Chunk { x, y }
    }

    #[inline]
    pub fn xy(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    #[inline]
    pub fn position_top_left_tile(&self) -> Tile {
        let center = self.position_center_tile();
        Tile {
            x: center.x - TILES_AROUND_CENTER,
            y: center.y - TILES_AROUND_CENTER,
        }
    }

    #[inline]
    pub fn position_top_right_tile(&self) -> Tile {
        let center = self.position_center_tile();
        Tile {
            x: center.x + TILES_AROUND_CENTER,
            y: center.y - TILES_AROUND_CENTER,
        }
    }

    #[inline]
    pub fn position_bottom_left_tile(&self) -> Tile {
        let center = self.position_center_tile();
        Tile {
            x: center.x - TILES_AROUND_CENTER,
            y: center.y + TILES_AROUND_CENTER,
        }
    }

    #[inline]
    pub fn position_bottom_right_tile(&self) -> Tile {
        let center = self.position_center_tile();
        Tile {
            x: center.x + TILES_AROUND_CENTER,
            y: center.y + TILES_AROUND_CENTER,
        }
    }

    #[inline]
    pub fn position_center_tile(&self) -> Tile {
        Tile {
            x: self.x * TILES_PER_CHUNK,
            y: self.y * TILES_PER_CHUNK,
        }
    }

    #[inline]
    pub fn child_tiles_range(&self) -> Range {
        let center = self.position_center_tile();
        Range::new(
            center.x - TILES_AROUND_CENTER,
            center.y - TILES_AROUND_CENTER,
            center.x + TILES_AROUND_CENTER,
            center.y + TILES_AROUND_CENTER,
        )
    }

    #[inline]
    pub fn has_tile(&self, tile: Tile) -> bool {
        self.child_tiles_range().has_tile(tile)
    }

    #[inline]
    pub fn area(&self) -> Area {
        Area {
            x: (((self.x as f32) + CHUNKS_PER_AREA_HALF) / (CHUNKS_PER_AREA as f32)).floor() as i32,
            y: (((self.y as f32) + CHUNKS_PER_AREA_HALF) / (CHUNKS_PER_AREA as f32)).floor() as i32,
        }
    }
}

#[auto_impl_ops::auto_ops]
impl AddAssign<&Chunk> for Chunk {
    fn add_assign(&mut self, other: &Self) {
        self.x = &self.x + &other.x;
        self.y = &self.y + &other.y;
    }
}

#[auto_impl_ops::auto_ops]
impl SubAssign<&Chunk> for Chunk {
    fn sub_assign(&mut self, other: &Self) {
        self.x = &self.x - &other.x;
        self.y = &self.y - &other.y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at() {
        assert_eq!(Chunk::at(4, 6), Chunk { x: 4, y: 6 });
    }

    #[test]
    fn test_center_tile() {
        assert_eq!(Chunk::at(-1, 0).position_center_tile(), Tile::at(-15, 0));
        assert_eq!(Chunk::at(0, 0).position_center_tile(), Tile::at(0, 0));
        assert_eq!(Chunk::at(1, 0).position_center_tile(), Tile::at(15, 0));
        assert_eq!(
            Chunk::at(-100, 200).position_center_tile(),
            Tile::at(-1500, 3000)
        );
    }

    #[test]
    fn test_child_tiles_range() {
        assert_eq!(
            Chunk::at(-1, -1).child_tiles_range(),
            Range::new(-22, -22, -8, -8)
        );
        assert_eq!(
            Chunk::at(0, 0).child_tiles_range(),
            Range::new(-7, -7, 7, 7)
        );
        assert_eq!(
            Chunk::at(1, 1).child_tiles_range(),
            Range::new(8, 8, 22, 22)
        );
    }

    #[test]
    fn test_has_tile() {
        assert!(Chunk::at(0, 0).has_tile(Tile::at(-7, -7)));
        assert!(Chunk::at(0, 0).has_tile(Tile::at(0, 0)));
        assert!(Chunk::at(0, 0).has_tile(Tile::at(7, 7)));
        assert!(!Chunk::at(0, 0).has_tile(Tile::at(8, 7)));
        assert!(!Chunk::at(0, 0).has_tile(Tile::at(7, 8)));
        assert!(Chunk::at(1, 0).has_tile(Tile::at(8, 7)));
        assert!(Chunk::at(0, 1).has_tile(Tile::at(7, 8)));
        assert!(Chunk::at(1, 1).has_tile(Tile::at(8, 8)));
        assert!(!Chunk::at(-1, -1).has_tile(Tile::at(8, 7)));
        assert!(!Chunk::at(-1, -1).has_tile(Tile::at(7, 8)));
        assert!(!Chunk::at(-1, -1).has_tile(Tile::at(8, 8)));
    }

    #[test]
    fn test_area() {
        assert_eq!(
            Chunk::at(-8, -8).area(),
            Area::at(-1, -1),
            "-1x-1_bottom_right"
        );
        assert_eq!(Chunk::at(-8, 0).area(), Area::at(-1, 0), "-1x0_top_right");
        assert_eq!(Chunk::at(-7, 0).area(), Area::at(0, 0), "0x0_top_left");
        assert_eq!(Chunk::at(0, 0).area(), Area::at(0, 0), "0x0_top_center");
        assert_eq!(Chunk::at(7, 0).area(), Area::at(0, 0), "0x0_top_right");
        assert_eq!(Chunk::at(8, 0).area(), Area::at(1, 0), "1x0_top_left");

        assert_eq!(Chunk::at(8, 8).area(), Area::at(1, 1), "1x1_top_left");
        assert_eq!(Chunk::at(15, 15).area(), Area::at(1, 1), "1x1_center");
        assert_eq!(Chunk::at(22, 22).area(), Area::at(1, 1), "1x1_bottom_right");

        assert_eq!(Chunk::at(293, 0).area(), Area::at(20, 0), "20x0_left");
        assert_eq!(Chunk::at(300, 0).area(), Area::at(20, 0), "20x0_center");
        assert_eq!(Chunk::at(307, 0).area(), Area::at(20, 0), "20x0_right");
        assert_eq!(Chunk::at(308, 0).area(), Area::at(21, 0), "21x0_left");
    }
}
