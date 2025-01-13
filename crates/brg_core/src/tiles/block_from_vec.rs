use super::block::{Area, Block, Chunk, Cluster, Tile};
use super::block_parent::BlockParent;
use crate::prelude::V2;

#[allow(dead_code)]
pub trait VecExt {
    fn tile(&self) -> Tile;
    fn chunk(&self) -> Chunk;
    fn area(&self) -> Area;
    fn cluster(&self) -> Cluster;
}

impl VecExt for V2 {
    #[inline(always)]
    fn tile(&self) -> Tile {
        Tile::at(self.x.floor() as i32, self.y.floor() as i32)
    }

    #[inline(always)]
    fn chunk(&self) -> Chunk {
        self.tile().parent()
    }

    #[inline(always)]
    fn area(&self) -> Area {
        self.chunk().parent()
    }

    #[inline(always)]
    fn cluster(&self) -> Cluster {
        self.area().parent()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_v2_owner() {
        assert_eq!(V2::new(-6.99, -7.0).tile(), Tile::at(-7, -7));
        assert_eq!(V2::new(-6.99, -7.0).chunk(), Chunk::at(0, 0));
        assert_eq!(V2::new(-7.01, -7.0).tile(), Tile::at(-8, -7));
        assert_eq!(V2::new(-7.01, -7.0).chunk(), Chunk::at(-1, 0));

        assert_eq!(V2::new(7.9, 6.0).chunk(), Chunk::at(0, 0));
        assert_eq!(V2::new(7.9, 6.0).area(), Area::at(0, 0));

        assert_eq!(V2::new(8.9, 6.0).chunk(), Chunk::at(1, 0));
        assert_eq!(V2::new(8.9, 6.0).area(), Area::at(0, 0));
        assert_eq!(V2::new(111.0, -224.0).area(), Area::at(0, -1));
        assert_eq!(V2::new(112.5, -224.0).area(), Area::at(0, -1));
        assert_eq!(V2::new(113.0, -224.0).area(), Area::at(1, -1));
    }
}
