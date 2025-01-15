use super::block::{Area, Block, Chunk, Cluster, Tile};
use super::block_range::Range;
use super::consts::*;

#[allow(dead_code)]
pub trait BlockChild {
    type Child: Block;

    fn child_elem_center(&self) -> Self::Child;
    fn child_elem_top_left(&self) -> Self::Child;
    fn child_elem_top_right(&self) -> Self::Child;
    fn child_elem_bottom_left(&self) -> Self::Child;
    fn child_elem_bottom_right(&self) -> Self::Child;
    fn child_range(&self) -> Range<Self::Child>;
}

macro_rules! impl_block_child {
    ($blockStruct:ident, $child:ident) => {
        impl BlockChild for $blockStruct {
            type Child = $child;

            #[inline(always)]
            fn child_elem_center(&self) -> Self::Child {
                Self::Child::at(self.x * T_LIB_CONT_WIDTH, self.y * T_LIB_CONT_HEIGHT)
            }

            #[inline(always)]
            fn child_elem_top_left(&self) -> Self::Child {
                let center = self.child_elem_center();
                Self::Child::at(
                    center.x - T_LIB_ELEMENTS_AROUND_CENTER,
                    center.y - T_LIB_ELEMENTS_AROUND_CENTER,
                )
            }

            #[inline(always)]
            fn child_elem_top_right(&self) -> Self::Child {
                let center = self.child_elem_center();
                Self::Child::at(
                    center.x + T_LIB_ELEMENTS_AROUND_CENTER,
                    center.y - T_LIB_ELEMENTS_AROUND_CENTER,
                )
            }

            #[inline(always)]
            fn child_elem_bottom_left(&self) -> Self::Child {
                let center = self.child_elem_center();
                Self::Child::at(
                    center.x - T_LIB_ELEMENTS_AROUND_CENTER,
                    center.y + T_LIB_ELEMENTS_AROUND_CENTER,
                )
            }

            #[inline(always)]
            fn child_elem_bottom_right(&self) -> Self::Child {
                let center = self.child_elem_center();
                Self::Child::at(
                    center.x + T_LIB_ELEMENTS_AROUND_CENTER,
                    center.y + T_LIB_ELEMENTS_AROUND_CENTER,
                )
            }

            #[inline]
            fn child_range(&self) -> Range<Self::Child> {
                let center = self.child_elem_center();
                Range::<Self::Child>::new(
                    center.x - T_LIB_ELEMENTS_AROUND_CENTER,
                    center.y - T_LIB_ELEMENTS_AROUND_CENTER,
                    center.x + T_LIB_ELEMENTS_AROUND_CENTER,
                    center.y + T_LIB_ELEMENTS_AROUND_CENTER,
                )
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
        assert_eq!(Chunk::at(0, 0).child_elem_center(), Tile::at(0, 0));
        assert_eq!(Chunk::at(1, 2).child_elem_center(), Tile::at(15, 30));
        assert_eq!(Area::at(1, 2).child_elem_center(), Chunk::at(15, 30));
        assert_eq!(Cluster::at(1, 2).child_elem_center(), Area::at(15, 30));
    }

    #[test]
    fn elem_top_left() {
        assert_eq!(Chunk::at(0, 0).child_elem_top_left(), Tile::at(-7, -7));
        assert_eq!(Area::at(0, 0).child_elem_top_left(), Chunk::at(-7, -7));
        assert_eq!(Cluster::at(1, 1).child_elem_top_left(), Area::at(8, 8));
    }

    #[test]
    fn elem_bottom_right() {
        assert_eq!(Chunk::at(0, 0).child_elem_bottom_right(), Tile::at(7, 7));
        assert_eq!(Area::at(0, 0).child_elem_bottom_right(), Chunk::at(7, 7));
        assert_eq!(
            Cluster::at(1, 1).child_elem_bottom_right(),
            Area::at(22, 22)
        );
    }

    #[test]
    fn child_range() {
        assert_eq!(
            Chunk::at(0, 0).child_range(),
            Range::<Tile>::new(-7, -7, 7, 7)
        );
        assert_eq!(
            Area::at(1, -1).child_range(),
            Range::<Chunk>::new(8, -22, 22, -8)
        );
    }
}
