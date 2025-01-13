use std::marker::PhantomData;

use bevy::prelude::Reflect;

pub trait Block {
    fn at(x: i32, y: i32) -> Self;
}

// only for internal use (like in range iter)
pub(super) trait BlockXY {
    fn x(&self) -> i32;
    fn set_x(&mut self, x: i32);

    fn y(&self) -> i32;
    fn set_y(&mut self, y: i32);
}

#[derive(Default, Eq, PartialEq, Debug, Copy, Clone, Reflect, Hash)]
pub struct BlockOf<T> {
    pub x:         i32,
    pub y:         i32,
    #[reflect(ignore)]
    _phantom_data: PhantomData<T>,
}

#[derive(Default, Eq, PartialEq, Debug, Copy, Clone, Reflect, Hash)]
pub struct _type_tile;
pub type Tile = BlockOf<_type_tile>;

#[derive(Default, Eq, PartialEq, Debug, Copy, Clone, Reflect, Hash)]
pub struct _type_chunk;
pub type Chunk = BlockOf<_type_chunk>;

#[derive(Default, Eq, PartialEq, Debug, Copy, Clone, Reflect, Hash)]
pub struct _type_area;
pub type Area = BlockOf<_type_area>;

#[derive(Default, Eq, PartialEq, Debug, Copy, Clone, Reflect, Hash)]
pub struct _type_cluster;
pub type Cluster = BlockOf<_type_cluster>;

macro_rules! impl_block {
    ($blockStruct:ident) => {
        impl Block for $blockStruct {
            #[inline(always)]
            fn at(x: i32, y: i32) -> $blockStruct {
                $blockStruct {
                    x,
                    y,
                    _phantom_data: Default::default(),
                }
            }
        }

        impl BlockXY for $blockStruct {
            #[inline(always)]
            fn x(&self) -> i32 {
                self.x
            }

            #[inline(always)]
            fn set_x(&mut self, x: i32) {
                self.x = x;
            }

            #[inline(always)]
            fn y(&self) -> i32 {
                self.y
            }

            #[inline(always)]
            fn set_y(&mut self, y: i32) {
                self.y = y;
            }
        }
    };
}

impl_block!(Tile);
impl_block!(Chunk);
impl_block!(Area);
impl_block!(Cluster);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_xy() {
        assert_eq!(Tile::at(1, 2), Tile {
            x:             1,
            y:             2,
            _phantom_data: Default::default(),
        });
        assert_eq!(Cluster::at(-21, 1293), Cluster {
            x:             -21,
            y:             1293,
            _phantom_data: Default::default(),
        });
    }
}
