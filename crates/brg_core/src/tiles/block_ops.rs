use std::ops::{Add, AddAssign, Sub, SubAssign};

use super::block::{Area, Chunk, Cluster, Tile};

macro_rules! impl_block_auto_ops {
    ($blockStruct:ident) => {
        #[auto_impl_ops::auto_ops]
        impl AddAssign<&$blockStruct> for $blockStruct {
            fn add_assign(&mut self, other: &Self) {
                self.x = &self.x + &other.x;
                self.y = &self.y + &other.y;
            }
        }

        #[auto_impl_ops::auto_ops]
        impl SubAssign<&$blockStruct> for $blockStruct {
            fn sub_assign(&mut self, other: &Self) {
                self.x = &self.x - &other.x;
                self.y = &self.y - &other.y;
            }
        }
    };
}

impl_block_auto_ops!(Tile);
impl_block_auto_ops!(Chunk);
impl_block_auto_ops!(Area);
impl_block_auto_ops!(Cluster);
