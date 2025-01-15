use super::block::{Area, Block, Chunk, Cluster, Tile};
use super::block_child::BlockChild;
use super::block_size::BlockSize;
use crate::prelude::V2;

#[allow(dead_code)]
pub trait BlockPosition {
    fn position(&self) -> V2;
    fn position_center(&self) -> V2;
    fn position_tr(&self) -> V2;
    fn position_bl(&self) -> V2;
    fn position_br(&self) -> V2;
}

impl BlockPosition for Tile {
    /// Tile top-left world position
    #[inline(always)]
    fn position(&self) -> V2 {
        V2 {
            x: self.x as f32,
            y: self.y as f32,
        }
    }

    /// Tile center position in absolute world coordinates
    #[inline(always)]
    fn position_center(&self) -> V2 {
        self.position() + 0.5
    }

    /// Tile top-right position in absolute world coordinates
    #[inline(always)]
    fn position_tr(&self) -> V2 {
        self.position() + V2 { x: 1.0, y: 0.0 }
    }

    /// Tile bottom-left position in absolute world coordinates
    #[inline(always)]
    fn position_bl(&self) -> V2 {
        self.position() + V2 { x: 0.0, y: 1.0 }
    }

    /// Tile bottom-right position in absolute world coordinates
    #[inline(always)]
    fn position_br(&self) -> V2 {
        self.position() + V2 { x: 1.0, y: 1.0 }
    }
}

macro_rules! impl_block_position_with_child {
    ($blockStruct:ty) => {
        impl BlockPosition for $blockStruct {
            /// Container top-left world position
            #[inline(always)]
            fn position(&self) -> V2 {
                self.child_elem_top_left().position()
            }

            /// Container center position in absolute world coordinates
            #[inline(always)]
            fn position_center(&self) -> V2 {
                self.child_elem_center().position_center()
            }

            /// Container top-right in absolute world coordinates
            #[inline(always)]
            fn position_tr(&self) -> V2 {
                self.child_elem_top_right().position_tr()
            }

            /// Container bottom-left position in absolute world coordinates
            #[inline(always)]
            fn position_bl(&self) -> V2 {
                self.child_elem_bottom_left().position_bl()
            }

            /// Container bottom-right position in absolute world coordinates
            #[inline(always)]
            fn position_br(&self) -> V2 {
                self.child_elem_bottom_right().position_br()
            }
        }
    };
}

impl_block_position_with_child!(Chunk);
impl_block_position_with_child!(Area);
impl_block_position_with_child!(Cluster);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tiles::block::Block;

    #[test]
    fn position_top_left() {
        assert_eq!(Tile::at(0, 0).position(), V2::new(0.0, 0.0));
        assert_eq!(Tile::at(1, 2).position(), V2::new(1.0, 2.0));
        assert_eq!(Tile::at(-8, -20).position(), V2::new(-8.0, -20.0));

        assert_eq!(Chunk::at(0, 0).position(), V2::new(-7.0, -7.0));
        assert_eq!(Chunk::at(1, 1).position(), V2::new(8.0, 8.0));

        assert_eq!(Area::at(0, 3).position(), V2::new(-112.0, 563.0));
    }

    #[test]
    fn position_center() {
        assert_eq!(Tile::at(0, 0).position_center(), V2::new(0.5, 0.5));
        assert_eq!(Tile::at(1, 2).position_center(), V2::new(1.5, 2.5));
        assert_eq!(Tile::at(-8, -20).position_center(), V2::new(-7.5, -19.5));

        assert_eq!(Chunk::at(0, 0).position_center(), V2::new(0.5, 0.5));
        assert_eq!(Chunk::at(1, 1).position_center(), V2::new(15.5, 15.5));

        assert_eq!(Area::at(0, 0).position_center(), V2::new(0.5, 0.5));
        assert_eq!(Area::at(1, 0).position_center(), V2::new(225.5, 0.5));
    }
}
