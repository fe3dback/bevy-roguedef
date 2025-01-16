use super::block::{Area, Chunk, Cluster, Tile};
use super::block_child::BlockChild;
use crate::prelude::V2;

#[allow(dead_code)]
pub trait BlockPosition {
    fn position_center(&self) -> V2;
    fn position_tl(&self) -> V2;
    fn position_tr(&self) -> V2;
    fn position_bl(&self) -> V2;
    fn position_br(&self) -> V2;
}

impl BlockPosition for Tile {
    /// Tile center position in absolute world coordinates
    #[inline(always)]
    fn position_center(&self) -> V2 {
        self.position_tl() + 0.5
    }

    /// Tile top-left world position
    #[inline(always)]
    fn position_tl(&self) -> V2 {
        V2 {
            x: self.x as f32,
            y: self.y as f32,
        }
    }

    /// Tile top-right position in absolute world coordinates
    #[inline(always)]
    fn position_tr(&self) -> V2 {
        self.position_tl() + V2 { x: 1.0, y: 0.0 }
    }

    /// Tile bottom-left position in absolute world coordinates
    #[inline(always)]
    fn position_bl(&self) -> V2 {
        self.position_tl() + V2 { x: 0.0, y: 1.0 }
    }

    /// Tile bottom-right position in absolute world coordinates
    #[inline(always)]
    fn position_br(&self) -> V2 {
        self.position_tl() + V2 { x: 1.0, y: 1.0 }
    }
}

macro_rules! impl_block_position_with_child {
    ($blockStruct:ty) => {
        impl BlockPosition for $blockStruct {
            /// Container center position in absolute world coordinates
            #[inline(always)]
            fn position_center(&self) -> V2 {
                self.position_tl() + Self::size_m() / 2.0
            }

            /// Container top-left world position
            #[inline(always)]
            fn position_tl(&self) -> V2 {
                self.child_elem_tl().position_tl()
            }

            /// Container top-right in absolute world coordinates
            #[inline(always)]
            fn position_tr(&self) -> V2 {
                self.child_elem_tr().position_tr()
            }

            /// Container bottom-left position in absolute world coordinates
            #[inline(always)]
            fn position_bl(&self) -> V2 {
                self.child_elem_bl().position_bl()
            }

            /// Container bottom-right position in absolute world coordinates
            #[inline(always)]
            fn position_br(&self) -> V2 {
                self.child_elem_br().position_br()
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
        assert_eq!(Tile::at(0, 0).position_tl(), V2::new(0.0, 0.0));
        assert_eq!(Tile::at(1, 2).position_tl(), V2::new(1.0, 2.0));
        assert_eq!(Tile::at(-8, -20).position_tl(), V2::new(-8.0, -20.0));

        assert_eq!(Chunk::at(0, 0).position_tl(), V2::new(0.0, 0.0));
        assert_eq!(Chunk::at(1, 1).position_tl(), V2::new(16.0, 16.0));

        assert_eq!(Area::at(0, 3).position_tl(), V2::new(0.0, 768.0));
        assert_eq!(Area::at(-3, 1).position_tl(), V2::new(-768.0, 256.0));
    }

    #[test]
    fn position_center() {
        assert_eq!(Tile::at(0, 0).position_center(), V2::new(0.5, 0.5));
        assert_eq!(Tile::at(1, 2).position_center(), V2::new(1.5, 2.5));
        assert_eq!(Tile::at(-8, -20).position_center(), V2::new(-7.5, -19.5));

        assert_eq!(Chunk::at(0, 0).position_center(), V2::new(8.0, 8.0));
        assert_eq!(Chunk::at(1, 1).position_center(), V2::new(24.0, 24.0));

        assert_eq!(Area::at(0, 0).position_center(), V2::new(128.0, 128.0));
        assert_eq!(Area::at(1, 0).position_center(), V2::new(384.0, 128.0));
    }
}
