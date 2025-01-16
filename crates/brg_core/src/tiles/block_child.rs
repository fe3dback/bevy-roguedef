use super::block::{Area, Block, Chunk, Cluster, Tile};
use super::block_range::Range;
use super::consts::*;

#[allow(dead_code)]
pub trait BlockChild {
    type Child: Block;

    fn child_elem_center(&self) -> Self::Child;
    fn child_elem_tl(&self) -> Self::Child;
    fn child_elem_tr(&self) -> Self::Child;
    fn child_elem_bl(&self) -> Self::Child;
    fn child_elem_br(&self) -> Self::Child;
    fn child_range(&self) -> Range<Self::Child>;
}

const WH: i32 = T_LIB_CONT_ROW_LEN as i32;
const WH_HALF: i32 = T_LIB_CONT_ROW_LEN_HALF as i32;

macro_rules! impl_block_child {
    ($blockStruct:ident, $child:ident) => {
        impl BlockChild for $blockStruct {
            type Child = $child;

            #[inline(always)]
            fn child_elem_tl(&self) -> Self::Child {
                Self::Child::at(self.x * WH, self.y * WH)
            }

            #[inline(always)]
            fn child_elem_center(&self) -> Self::Child {
                let tl = self.child_elem_tl();
                Self::Child::at(tl.x + WH_HALF, tl.y + WH_HALF)
            }

            #[inline(always)]
            fn child_elem_tr(&self) -> Self::Child {
                let tl = self.child_elem_tl();
                Self::Child::at(tl.x + WH - 1, tl.y)
            }

            #[inline(always)]
            fn child_elem_bl(&self) -> Self::Child {
                let tl = self.child_elem_tl();
                Self::Child::at(tl.x, tl.y + WH - 1)
            }

            #[inline(always)]
            fn child_elem_br(&self) -> Self::Child {
                let tl = self.child_elem_tl();
                Self::Child::at(tl.x + WH - 1, tl.y + WH - 1)
            }

            #[inline]
            fn child_range(&self) -> Range<Self::Child> {
                let tl = self.child_elem_tl();
                let br = self.child_elem_br();
                Range::<Self::Child>::new(tl.x, tl.y, br.x, br.y)
            }
        }
    };
}

impl_block_child!(Chunk, Tile);
impl_block_child!(Area, Chunk);
impl_block_child!(Cluster, Area);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn elem_center() {
        assert_eq!(Chunk::at(0, 0).child_elem_center(), Tile::at(8, 8));
        assert_eq!(Chunk::at(-1, -2).child_elem_center(), Tile::at(-8, -24));
        assert_eq!(Chunk::at(1, 2).child_elem_center(), Tile::at(24, 40));
        assert_eq!(Area::at(1, 2).child_elem_center(), Chunk::at(24, 40));
        assert_eq!(Cluster::at(1, 2).child_elem_center(), Area::at(24, 40));
    }

    #[test]
    fn elem_top_left() {
        assert_eq!(Chunk::at(0, 0).child_elem_tl(), Tile::at(0, 0));
        assert_eq!(Area::at(0, 0).child_elem_tl(), Chunk::at(0, 0));
        assert_eq!(Cluster::at(1, -1).child_elem_tl(), Area::at(16, -16));

        assert_eq!(Chunk::at(0, 0).child_elem_tr(), Tile::at(15, 0));
        assert_eq!(Chunk::at(1, 0).child_elem_tl(), Tile::at(16, 0));
    }

    #[test]
    fn elem_top_right() {
        assert_eq!(Chunk::at(-2, -2).child_elem_br(), Tile::at(-17, -17));
        assert_eq!(Chunk::at(-1, -1).child_elem_tl(), Tile::at(-16, -16));
        assert_eq!(Chunk::at(-1, -1).child_elem_br(), Tile::at(-1, -1));
        assert_eq!(Chunk::at(0, 0).child_elem_tl(), Tile::at(0, 0));
        assert_eq!(Chunk::at(0, 0).child_elem_br(), Tile::at(15, 15));
        assert_eq!(Chunk::at(1, 1).child_elem_tl(), Tile::at(16, 16));
        assert_eq!(Chunk::at(1, 1).child_elem_br(), Tile::at(31, 31));
        assert_eq!(Chunk::at(2, 2).child_elem_tl(), Tile::at(32, 32));
    }

    #[test]
    fn elem_bottom_right() {
        assert_eq!(Chunk::at(0, 0).child_elem_br(), Tile::at(15, 15));
        assert_eq!(Area::at(0, 0).child_elem_br(), Chunk::at(15, 15));
        assert_eq!(Cluster::at(1, -1).child_elem_br(), Area::at(31, -1));
    }

    #[test]
    fn child_range() {
        assert_eq!(
            Chunk::at(0, 0).child_range(),
            Range::<Tile>::new(0, 0, 15, 15)
        );
        assert_eq!(
            Area::at(1, -1).child_range(),
            Range::<Chunk>::new(16, -16, 31, -1)
        );
    }
}
